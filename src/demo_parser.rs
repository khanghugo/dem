use std::{ffi::OsStr, fs::OpenOptions, io::Read, path::Path};

use nom::{
    bytes::complete::take,
    combinator::{map, verify},
    multi::{count, many0, many_till},
    number::complete::{le_f32, le_i16, le_i32, le_i8, le_u16, le_u32, le_u8},
    sequence::tuple,
};

use crate::{
    nom_helper::{nom_fail, take_point_float, Result},
    parse_netmsg,
    types::{
        Aux, AuxRefCell, ClientData, ConsoleCommand, Demo, DemoBuffer, DemoInfo, Directory,
        DirectoryEntry, Event, EventArgs, Frame, FrameData, Header, MessageData, MoveVars,
        NetworkMessage, NetworkMessageType, RefParams, SequenceInfo, Sound, UserCmd,
        WeaponAnimation,
    },
};

impl Demo {
    /// It is faster to parse netmessage because it just is for some reasons
    pub fn parse_from_file(
        path: impl AsRef<OsStr> + AsRef<Path>,
        should_parse_netmessage: bool,
    ) -> eyre::Result<Self> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut bytes: Vec<u8> = vec![];

        file.read_to_end(&mut bytes)?;

        Self::parse_from_bytes(bytes.as_slice(), should_parse_netmessage)
    }

    pub fn parse_from_bytes(
        demo_bytes: &[u8],
        should_parse_netmessage: bool,
    ) -> eyre::Result<Self> {
        match parse_demo(demo_bytes, should_parse_netmessage) {
            Ok((_, demo)) => Ok(demo),
            Err(err) => Err(eyre::eyre!("cannot parse demo: {}", err)),
        }
    }
}

pub fn parse_demo(i: &[u8], should_parse_netmessage: bool) -> Result<Demo> {
    let aux2 = Aux::new2();

    let file_start = i;

    let (i, header) = parse_header(i)?;

    let (i, directory) = if header.directory_offset == 0 {
        let frames_start = i;

        parse_fallback_directory(
            frames_start,
            file_start,
            should_parse_netmessage,
            aux2.clone(),
        )
    } else {
        let directory_start = &file_start[header.directory_offset as usize..];

        parse_directory(
            directory_start,
            file_start,
            should_parse_netmessage,
            aux2.clone(),
        )
    }?;

    Ok((
        i,
        Demo {
            header,
            directory,
            _aux: Some(aux2),
        },
    ))
}

pub fn parse_header(i: &[u8]) -> Result<Header> {
    let (i, magic) = take(8usize)(i)?;

    if magic != "HLDEMO\x00\x00".as_bytes() {
        return nom_fail(format!("magic is not HLDEMO: `{:?}`", magic));
    }

    map(
        tuple((
            le_i32,
            le_i32,
            take(260usize),
            take(260usize),
            le_u32,
            le_i32,
        )),
        |(
            demo_protocol,
            network_protocol,
            map_name,
            game_directory,
            map_checksum,
            directory_offset,
        ): (_, _, &[u8], &[u8], _, _)| Header {
            magic: magic.to_vec(),
            demo_protocol,
            network_protocol,
            map_name: map_name.to_vec(),
            game_directory: game_directory.to_vec(),
            map_checksum,
            directory_offset,
        },
    )(i)
}

pub fn parse_directory<'a>(
    i: &'a [u8],
    file_start: &'a [u8],
    should_parse_netmessage: bool,
    aux: AuxRefCell,
) -> Result<'a, Directory> {
    let (i, entry_count) = le_u32(i)?;

    let local_parse_directory_entry =
        |i| parse_directory_entry(i, file_start, should_parse_netmessage, aux.clone());

    map(
        count(local_parse_directory_entry, entry_count as usize),
        |entries| Directory { entries },
    )(i)
}

/// Parse a fallback directory for demo files that were not finalized by a client.
///
/// Unfinalized demos have the following differences from finalized demos:
///
/// - Directory metadata is not yet written
///   - Directory offset is 0
///   - Number of entries is 0
///   - Entry metadata is missing
///  - A terminating NextSection frame for the current entry is not yet written
///
/// This parser reconstructs the missing details from frame data so they can be organized into
/// individual directory entries.
pub fn parse_fallback_directory<'a>(
    frames_start: &'a [u8],
    file_start: &'a [u8],
    should_parse_netmessage: bool,
    aux: AuxRefCell,
) -> Result<'a, Directory> {
    let parser = |i| parse_frame(i, should_parse_netmessage, aux.clone());

    let loading_entry_start = frames_start;

    let (i, (mut loading_frames, next_section_frame)) = many_till(
        parser,
        verify(parser, |frame| {
            matches!(frame.frame_data, FrameData::NextSection)
        }),
    )(frames_start)?;

    loading_frames.push(next_section_frame);

    let loading_entry_end = i;

    let loading_entry = DirectoryEntry {
        type_: 0,
        description: format!("{:\x00<64}", "LOADING").into(),
        flags: -1,
        cd_track: -1,
        track_time: 0.,

        frame_count: loading_frames.len() as i32,
        frame_offset: (file_start.len() - loading_entry_start.len()) as i32,
        file_length: (loading_entry_start.len() - loading_entry_end.len()) as i32,

        frames: loading_frames,
    };

    let playback_entry_start = i;
    let (i, playback_frames) = many0(parser)(i)?;
    let playback_entry_end = i;

    let playback_entry = DirectoryEntry {
        type_: 1,
        description: format!("{:\x00<64}", "Playback").into(),
        flags: -1,
        cd_track: -1,
        track_time: 0.,

        frame_count: playback_frames.len() as i32,
        frame_offset: (file_start.len() - playback_entry_start.len()) as i32,
        file_length: (playback_entry_start.len() - playback_entry_end.len()) as i32,

        frames: playback_frames,
    };

    Ok((
        i,
        Directory {
            entries: vec![loading_entry, playback_entry],
        },
    ))
}

pub fn parse_directory_entry<'a>(
    i: &'a [u8],
    file_start: &'a [u8],
    should_parse_netmessage: bool,
    aux: AuxRefCell,
) -> Result<'a, DirectoryEntry> {
    let (
        end_of_current_directory_entry,
        (type_, description, flags, cd_track, track_time, frame_count, frame_offset, file_length),
    ) = tuple((
        le_i32,
        take(64usize),
        le_i32,
        le_i32,
        le_f32,
        le_i32,
        le_i32,
        le_i32,
    ))(i)?;

    // frame_count is unreliable
    // parse until NextSection and stop for current entry
    let mut frames: Vec<Frame> = vec![];
    let mut frames_start = &file_start[frame_offset as usize..];

    loop {
        let (end_current_frame, frame) =
            parse_frame(frames_start, should_parse_netmessage, aux.clone())?;

        let is_next_section = matches!(frame.frame_data, FrameData::NextSection);

        frames_start = end_current_frame;
        frames.push(frame);

        if is_next_section {
            break;
        }
    }

    Ok((
        end_of_current_directory_entry,
        DirectoryEntry {
            type_,
            description: description.to_vec(),
            flags,
            cd_track,
            track_time,
            frame_count,
            frame_offset,
            file_length,
            frames,
        },
    ))
}

pub fn parse_frame(i: &[u8], should_parse_netmessage: bool, aux: AuxRefCell) -> Result<Frame> {
    let (i, (type_, time, frame)) = tuple((le_u8, le_f32, le_i32))(i)?;

    let (i, frame_data) = match type_ {
        2 => (i, FrameData::DemoStart),
        3 => map(parse_console_command, FrameData::ConsoleCommand)(i)?,
        4 => map(parse_client_data, FrameData::ClientData)(i)?,
        5 => (i, FrameData::NextSection),
        6 => map(parse_event, FrameData::Event)(i)?,
        7 => map(parse_weapon_animation, FrameData::WeaponAnimation)(i)?,
        8 => map(parse_sound, FrameData::Sound)(i)?,
        9 => map(parse_demo_buffer, FrameData::DemoBuffer)(i)?,
        rest => {
            let (i, res) = parse_network_messages(i, should_parse_netmessage, aux)?;
            (
                i,
                FrameData::NetworkMessage(Box::new((
                    NetworkMessageType::try_from(rest).unwrap(),
                    res,
                ))),
            )
        }
    };

    Ok((
        i,
        Frame {
            time,
            frame,
            frame_data,
        },
    ))
}

pub fn parse_console_command(i: &[u8]) -> Result<ConsoleCommand> {
    map(take(64usize), |command: &[u8]| ConsoleCommand {
        command: command.to_vec(),
    })(i)
}

pub fn parse_client_data(i: &[u8]) -> Result<ClientData> {
    map(
        tuple((take_point_float, take_point_float, le_i32, le_f32)),
        |(origin, viewangles, weapon_bits, fov)| ClientData {
            origin,
            viewangles,
            weapon_bits,
            fov,
        },
    )(i)
}

pub fn parse_event(i: &[u8]) -> Result<Event> {
    map(
        tuple((le_i32, le_i32, le_f32, parse_event_args)),
        |(flags, index, delay, args)| Event {
            flags,
            index,
            delay,
            args,
        },
    )(i)
}

pub fn parse_event_args(i: &[u8]) -> Result<EventArgs> {
    map(
        tuple((
            le_i32,
            le_i32,
            take_point_float,
            take_point_float,
            take_point_float,
            le_i32,
            le_f32,
            le_f32,
            le_i32,
            le_i32,
            le_i32,
            le_i32,
        )),
        |(
            flags,
            entity_index,
            origin,
            angles,
            velocity,
            ducking,
            fparam1,
            fparam2,
            iparam1,
            iparam2,
            bparam1,
            bparam2,
        )| EventArgs {
            flags,
            entity_index,
            origin,
            angles,
            velocity,
            ducking,
            fparam1,
            fparam2,
            iparam1,
            iparam2,
            bparam1,
            bparam2,
        },
    )(i)
}

pub fn parse_weapon_animation(i: &[u8]) -> Result<WeaponAnimation> {
    map(tuple((le_i32, le_i32)), |(anim, body)| WeaponAnimation {
        anim,
        body,
    })(i)
}

pub fn parse_sound(i: &[u8]) -> Result<Sound> {
    let (i, (channel, sample_length)) = tuple((le_i32, le_u32))(i)?;

    // cannot return res directly because it is a closure and `channel` is outside of it

    #[allow(clippy::let_and_return)]
    let res = map(
        tuple((take(sample_length), le_f32, le_f32, le_i32, le_i32)),
        |(sample, attenuation, volume, flags, pitch): (&[u8], _, _, _, _)| Sound {
            channel,
            sample: sample.to_vec(),
            attenuation,
            volume,
            flags,
            pitch,
        },
    )(i);

    res
}

pub fn parse_demo_buffer(i: &[u8]) -> Result<DemoBuffer> {
    let (i, buffer_length) = le_u32(i)?;

    map(take(buffer_length), |buffer: &[u8]| DemoBuffer {
        buffer: buffer.to_vec(),
    })(i)
}

pub fn parse_network_messages(
    i: &[u8],
    should_parse_netmessage: bool,
    aux: AuxRefCell,
) -> Result<NetworkMessage> {
    let (i, (info, sequence_info, message_length)) =
        tuple((parse_network_messages_info, parse_sequence_info, le_u32))(i)?;

    if message_length > 65536 {
        return nom_fail(format!("message length too long: {}", message_length));
    }

    // let (i, netmessage_data_chunk) = take(message_length)(i)?;
    let netmessage_data_chunk = &i[..message_length as usize];
    let the_rest = &i[message_length as usize..];
    // let (i, netmessage_data_chunk) = count(le_u8, message_length as usize)(i)?;

    let messages = if should_parse_netmessage {
        // only parse the chunk
        // otherwise, it might spill outside, which it will
        let (_, netmessages) = parse_netmsg(netmessage_data_chunk, aux)?;

        MessageData::Parsed(netmessages)
    } else {
        MessageData::Raw(netmessage_data_chunk.to_vec())
    };

    Ok((
        the_rest,
        NetworkMessage {
            info,
            sequence_info,
            message_length,
            messages,
        },
    ))
}

pub fn parse_network_messages_info(i: &[u8]) -> Result<DemoInfo> {
    map(
        tuple((
            le_f32,
            parse_refparams,
            parse_usercmd,
            parse_movevars,
            take_point_float,
            le_i32,
        )),
        |(timestamp, refparams, usercmd, movevars, view, viewmodel)| DemoInfo {
            timestamp,
            refparams,
            usercmd,
            movevars,
            view,
            viewmodel,
        },
    )(i)
}

pub fn parse_refparams(i: &[u8]) -> Result<RefParams> {
    map(
        tuple((
            tuple((
                take_point_float,
                take_point_float,
                take_point_float,
                take_point_float,
                take_point_float,
            )),
            le_f32,
            le_f32,
            tuple((le_i32, le_i32, le_i32, le_i32)),
            le_i32,
            tuple((take_point_float, take_point_float)),
            tuple((
                take_point_float,
                le_f32,
                take_point_float,
                le_i32,
                take_point_float,
                le_f32,
                take_point_float,
            )),
            tuple((
                le_i32,
                le_i32,
                le_i32,
                le_i32,
                le_i32,
                le_i32,
                le_i32,
                le_i32,
                le_i32,
                count(le_i32, 4),
                le_i32,
                le_i32,
            )),
        )),
        |(
            (view_origin, view_angles, forward, right, up),
            frame_time,
            time,
            (intermission, paused, spectator, on_ground),
            water_level,
            (sim_vel, sim_org),
            (
                view_height,
                ideal_pitch,
                cl_viewangles,
                health,
                crosshair_angle,
                view_size,
                punch_angle,
            ),
            (
                max_clients,
                view_entity,
                player_num,
                max_entities,
                demo_playback,
                hardware,
                smoothing,
                ptr_cmd,
                ptr_move_vars,
                view_port,
                next_view,
                only_client_draw,
            ),
        )| RefParams {
            view_origin,
            view_angles,
            forward,
            right,
            up,
            frame_time,
            time,
            intermission,
            paused,
            spectator,
            on_ground,
            water_level,
            sim_vel,
            sim_org,
            view_height,
            ideal_pitch,
            cl_viewangles,
            health,
            crosshair_angle,
            view_size,
            punch_angle,
            max_clients,
            view_entity,
            player_num,
            max_entities,
            demo_playback,
            hardware,
            smoothing,
            ptr_cmd,
            ptr_move_vars,
            view_port,
            next_view,
            only_client_draw,
        },
    )(i)
}

pub fn parse_usercmd(i: &[u8]) -> Result<UserCmd> {
    map(
        tuple((
            le_i16,
            le_u8,
            le_u8,
            take_point_float,
            le_f32,
            le_f32,
            le_f32,
            le_i8,
            le_u8,
            le_u16,
            le_i8,
            le_i8,
            le_u8,
            le_u8,
            le_i32,
            take_point_float,
        )),
        |(
            lerp_msec,
            msec,
            unknown1,
            view_angles,
            forward_move,
            side_move,
            up_move,
            light_level,
            unknonwn2,
            buttons,
            impulse,
            weapon_select,
            unknown3,
            unknown4,
            impact_index,
            impact_position,
        )| UserCmd {
            lerp_msec,
            msec,
            unknown1,
            view_angles,
            forward_move,
            side_move,
            up_move,
            light_level,
            unknonwn2,
            buttons,
            impulse,
            weapon_select,
            unknown3,
            unknown4,
            impact_index,
            impact_position,
        },
    )(i)
}

pub fn parse_movevars(i: &[u8]) -> Result<MoveVars> {
    map(
        tuple((
            le_f32,
            tuple((le_f32, le_f32, le_f32)),
            tuple((
                le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32,
            )),
            le_f32,
            le_f32,
            le_i32,
            take(32usize),
            tuple((le_f32, le_f32)),
            tuple((take_point_float, take_point_float)),
        )),
        |(
            gravity,
            (stopspeed, maxspeed, spectatormaxspeed),
            (
                accelerate,
                airaccelerate,
                wateraccelerate,
                friction,
                edgefriction,
                waterfriction,
                entgravity,
                bounce,
                stepsize,
                maxvelocity,
            ),
            zmax,
            wave_height,
            footsteps,
            sky_name,
            (rollangle, rollspeed),
            (skycolor, skyvec),
        ): (_, _, _, _, _, _, &[u8], _, _)| MoveVars {
            gravity,
            stopspeed,
            maxspeed,
            spectatormaxspeed,
            accelerate,
            airaccelerate,
            wateraccelerate,
            friction,
            edgefriction,
            waterfriction,
            entgravity,
            bounce,
            stepsize,
            maxvelocity,
            zmax,
            wave_height,
            footsteps,
            sky_name: sky_name.to_vec(),
            rollangle,
            rollspeed,
            skycolor,
            skyvec,
        },
    )(i)
}

pub fn parse_sequence_info(i: &[u8]) -> Result<SequenceInfo> {
    map(
        tuple((le_i32, le_i32, le_i32, le_i32, le_i32, le_i32, le_i32)),
        |(
            incoming_sequence,
            incoming_acknowledged,
            incoming_reliable_acknowledged,
            incoming_reliable_sequence,
            outgoing_sequence,
            reliable_sequence,
            last_reliable_sequence,
        )| SequenceInfo {
            incoming_sequence,
            incoming_acknowledged,
            incoming_reliable_acknowledged,
            incoming_reliable_sequence,
            outgoing_sequence,
            reliable_sequence,
            last_reliable_sequence,
        },
    )(i)
}
