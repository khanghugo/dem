use std::{ffi::OsStr, fs::OpenOptions, io::Write, path::Path};

use crate::{
    byte_writer::ByteWriter,
    types::{Demo, FrameData, MessageData, NetworkMessageType},
};

impl Demo {
    pub fn write_to_file(&self, path: impl AsRef<OsStr> + AsRef<Path>) -> eyre::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        file.write_all(self.write_to_bytes().as_slice())?;
        file.flush()?;

        Ok(())
    }

    pub fn write_to_bytes(&self) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        // Magic has 8 bytes in total
        writer.append_u8_slice("HLDEMO\x00\x00".as_bytes());

        // header
        writer.append_i32(self.header.demo_protocol);
        writer.append_i32(self.header.network_protocol);
        writer.append_u8_slice(self.header.map_name.padded(260).as_slice());
        writer.append_u8_slice(self.header.game_directory.padded(260).as_slice());
        writer.append_u32(self.header.map_checksum);

        // directory
        // Delay writing directory offset
        let directory_offset_pos = writer.get_offset();
        writer.append_i32(0i32);

        let mut entry_offsets: Vec<(usize, usize)> = vec![];

        for entry in &self.directory.entries {
            let mut has_written_next_section = false;

            let entry_offset_start = writer.get_offset();

            for frame in &entry.frames {
                match frame.frame_data {
                    FrameData::DemoStart => writer.append_u8(2u8),
                    FrameData::ConsoleCommand(_) => writer.append_u8(3u8),
                    FrameData::ClientData(_) => writer.append_u8(4u8),
                    FrameData::NextSection => writer.append_u8(5u8),
                    FrameData::Event(_) => writer.append_u8(6u8),
                    FrameData::WeaponAnimation(_) => writer.append_u8(7u8),
                    FrameData::Sound(_) => writer.append_u8(8u8),
                    FrameData::DemoBuffer(_) => writer.append_u8(9u8),
                    FrameData::NetworkMessage(ref box_type) => match box_type.as_ref().0 {
                        NetworkMessageType::Start => writer.append_u8(0u8),
                        NetworkMessageType::Normal => writer.append_u8(1u8),
                        NetworkMessageType::Unknown(what) => writer.append_u8(what),
                    },
                }

                writer.append_f32(frame.time);
                writer.append_i32(frame.frame);

                // write the frame
                match &frame.frame_data {
                    FrameData::DemoStart => (),
                    FrameData::ConsoleCommand(frame) => {
                        writer.append_u8_slice(frame.command.padded(64).as_slice())
                    }
                    FrameData::ClientData(frame) => {
                        writer.append_f32_slice(frame.origin.as_slice());
                        writer.append_f32_slice(frame.viewangles.as_slice());
                        writer.append_i32(frame.weapon_bits);
                        writer.append_f32(frame.fov);
                    }
                    FrameData::NextSection => (),
                    FrameData::Event(frame) => {
                        writer.append_i32(frame.flags);
                        writer.append_i32(frame.index);
                        writer.append_f32(frame.delay);

                        writer.append_i32(frame.args.flags);
                        writer.append_i32(frame.args.entity_index);
                        writer.append_f32_slice(frame.args.origin.as_slice());
                        writer.append_f32_slice(frame.args.angles.as_slice());
                        writer.append_f32_slice(frame.args.velocity.as_slice());
                        writer.append_i32(frame.args.ducking);
                        writer.append_f32(frame.args.fparam1);
                        writer.append_f32(frame.args.fparam2);
                        writer.append_i32(frame.args.iparam1);
                        writer.append_i32(frame.args.iparam2);
                        writer.append_i32(frame.args.bparam1);
                        writer.append_i32(frame.args.bparam2);
                    }
                    FrameData::WeaponAnimation(frame) => {
                        writer.append_i32(frame.anim);
                        writer.append_i32(frame.body);
                    }
                    FrameData::Sound(frame) => {
                        writer.append_i32(frame.channel);
                        writer.append_i32(frame.sample.0.len() as i32);
                        writer.append_u8_slice(frame.sample.as_slice());
                        writer.append_f32(frame.attenuation);
                        writer.append_f32(frame.volume);
                        writer.append_i32(frame.flags);
                        writer.append_i32(frame.pitch);
                    }
                    FrameData::DemoBuffer(frame) => {
                        writer.append_i32(frame.buffer.len() as i32);
                        writer.append_u8_slice(frame.buffer.as_slice());
                    }
                    FrameData::NetworkMessage(ref box_type) => {
                        let data = &box_type.as_ref().1;

                        writer.append_f32(data.info.timestamp);
                        // ref_params
                        writer.append_f32_slice(data.info.refparams.view_origin.as_slice());
                        writer.append_f32_slice(data.info.refparams.view_angles.as_slice());
                        writer.append_f32_slice(data.info.refparams.forward.as_slice());
                        writer.append_f32_slice(data.info.refparams.right.as_slice());
                        writer.append_f32_slice(data.info.refparams.up.as_slice());
                        writer.append_f32(data.info.refparams.frame_time);
                        writer.append_f32(data.info.refparams.time);
                        writer.append_i32(data.info.refparams.intermission);
                        writer.append_i32(data.info.refparams.paused);
                        writer.append_i32(data.info.refparams.spectator);
                        writer.append_i32(data.info.refparams.on_ground);
                        writer.append_i32(data.info.refparams.water_level);
                        writer.append_f32_slice(data.info.refparams.sim_vel.as_slice());
                        writer.append_f32_slice(data.info.refparams.sim_org.as_slice());
                        writer.append_f32_slice(data.info.refparams.view_height.as_slice());
                        writer.append_f32(data.info.refparams.ideal_pitch);
                        writer.append_f32_slice(data.info.refparams.cl_viewangles.as_slice());
                        writer.append_i32(data.info.refparams.health);
                        writer.append_f32_slice(data.info.refparams.crosshair_angle.as_slice());
                        writer.append_f32(data.info.refparams.view_size);
                        writer.append_f32_slice(data.info.refparams.punch_angle.as_slice());
                        writer.append_i32(data.info.refparams.max_clients);
                        writer.append_i32(data.info.refparams.view_entity);
                        writer.append_i32(data.info.refparams.player_num);
                        writer.append_i32(data.info.refparams.max_entities);
                        writer.append_i32(data.info.refparams.demo_playback);
                        writer.append_i32(data.info.refparams.hardware);
                        writer.append_i32(data.info.refparams.smoothing);
                        writer.append_i32(data.info.refparams.ptr_cmd);
                        writer.append_i32(data.info.refparams.ptr_move_vars);
                        writer.append_i32_slice(data.info.refparams.view_port.as_slice());
                        writer.append_i32(data.info.refparams.next_view);
                        writer.append_i32(data.info.refparams.only_client_draw);
                        // usercmd
                        writer.append_i16(data.info.usercmd.lerp_msec);
                        writer.append_u8(data.info.usercmd.msec);
                        writer.append_u8(0u8); // unknown
                        writer.append_f32_slice(data.info.usercmd.view_angles.as_slice());
                        writer.append_f32(data.info.usercmd.forward_move);
                        writer.append_f32(data.info.usercmd.side_move);
                        writer.append_f32(data.info.usercmd.up_move);
                        writer.append_i8(data.info.usercmd.light_level);
                        writer.append_u8(0u8); // unknown
                        writer.append_u16(data.info.usercmd.buttons);
                        writer.append_i8(data.info.usercmd.impulse);
                        writer.append_i8(data.info.usercmd.weapon_select);
                        writer.append_u8(0u8); // unknown
                        writer.append_u8(0u8); // unknown
                        writer.append_i32(data.info.usercmd.impact_index);
                        writer.append_f32_slice(data.info.usercmd.impact_position.as_slice());
                        // movevars
                        writer.append_f32(data.info.movevars.gravity);
                        writer.append_f32(data.info.movevars.stopspeed);
                        writer.append_f32(data.info.movevars.maxspeed);
                        writer.append_f32(data.info.movevars.spectatormaxspeed);
                        writer.append_f32(data.info.movevars.accelerate);
                        writer.append_f32(data.info.movevars.airaccelerate);
                        writer.append_f32(data.info.movevars.wateraccelerate);
                        writer.append_f32(data.info.movevars.friction);
                        writer.append_f32(data.info.movevars.edgefriction);
                        writer.append_f32(data.info.movevars.waterfriction);
                        writer.append_f32(data.info.movevars.entgravity);
                        writer.append_f32(data.info.movevars.bounce);
                        writer.append_f32(data.info.movevars.stepsize);
                        writer.append_f32(data.info.movevars.maxvelocity);
                        writer.append_f32(data.info.movevars.zmax);
                        writer.append_f32(data.info.movevars.wave_height);
                        writer.append_i32(data.info.movevars.footsteps);
                        writer.append_u8_slice(data.info.movevars.sky_name.padded(32).as_slice());
                        writer.append_f32(data.info.movevars.rollangle);
                        writer.append_f32(data.info.movevars.rollspeed);
                        writer.append_f32(data.info.movevars.skycolor[0]);
                        writer.append_f32(data.info.movevars.skycolor[1]);
                        writer.append_f32(data.info.movevars.skycolor[2]);
                        writer.append_f32(data.info.movevars.skyvec[0]);
                        writer.append_f32(data.info.movevars.skyvec[1]);
                        writer.append_f32(data.info.movevars.skyvec[2]);
                        // still in info
                        writer.append_f32_slice(data.info.view.as_slice());
                        writer.append_i32(data.info.viewmodel);
                        // now other data
                        writer.append_i32(data.sequence_info.incoming_sequence);
                        writer.append_i32(data.sequence_info.incoming_acknowledged);
                        writer.append_i32(data.sequence_info.incoming_reliable_acknowledged);
                        writer.append_i32(data.sequence_info.incoming_reliable_sequence);
                        writer.append_i32(data.sequence_info.outgoing_sequence);
                        writer.append_i32(data.sequence_info.reliable_sequence);
                        writer.append_i32(data.sequence_info.last_reliable_sequence);

                        // write the frame itself
                        match &data.messages {
                            MessageData::Parsed(vec) => {
                                // delay writing message length
                                let start_offset_value = writer.get_offset();
                                writer.append_u32(0);

                                let start_length = writer.get_offset();

                                for message in vec {
                                    writer.append_u8_slice(
                                        message
                                            .write(self._aux.as_ref().unwrap().clone())
                                            .as_slice(),
                                    );
                                }

                                let end_length = writer.get_offset();

                                // this should be a function, wtf
                                writer.data.splice(
                                    start_offset_value..start_offset_value + 4,
                                    ((end_length - start_length) as u32).to_le_bytes(),
                                );
                            }
                            MessageData::Raw(vec) => {
                                writer.append_i32(vec.len() as i32);
                                writer.append_u8_slice(vec.as_slice());
                            }
                            MessageData::None => {
                                // length
                                writer.append_i32(0);
                            }
                        }
                    }
                }

                if matches!(frame.frame_data, FrameData::NextSection) {
                    has_written_next_section = true;
                }
            }

            if !has_written_next_section {
                writer.append_u8(5u8);
                writer.append_f32(0.);
                writer.append_i32(0);
            }

            entry_offsets.push((entry_offset_start, writer.get_offset()));
        }

        // writing the directory entry at the end because now we have the offset
        let directory_offset = writer.get_offset();

        writer.append_i32(self.directory.entries.len() as i32);

        for (entry, (offset_start, offset_end)) in
            self.directory.entries.iter().zip(entry_offsets.iter())
        {
            writer.append_i32(entry.type_);
            writer.append_u8_slice(entry.description.padded(64).as_slice());
            writer.append_i32(entry.flags);
            writer.append_i32(entry.cd_track);
            writer.append_f32(entry.track_time);

            writer.append_i32(entry.frames.len() as i32);
            writer.append_i32(*offset_start as i32);
            writer.append_i32((offset_end - offset_start) as i32);
        }

        writer.data.splice(
            directory_offset_pos..directory_offset_pos + 4,
            (directory_offset as u32).to_le_bytes(),
        );

        writer.data
    }
}
