use std::{fs, io::Write};

use hldemo::{Demo, Directory, DirectoryEntry, Frame, FrameData, Header, NetMsgFrameType};

use self::byte_writer::ByteWriter;

use super::*;

#[allow(dead_code)]
pub struct DemoWriter {
    pub filename: String,
    writer: ByteWriter,
}

#[allow(dead_code)]
impl DemoWriter {
    pub fn new(filename: String) -> DemoWriter {
        DemoWriter {
            filename,
            writer: ByteWriter::new(),
        }
    }

    pub fn write_file(&mut self, demo: Demo) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.filename)
            .unwrap();

        self.write_demo(demo);

        let _ = file.write_all(&self.writer.data);
    }

    fn write_demo(&mut self, demo: Demo) {
        // Magic has 8 bytes in total
        self.writer.append_u8_slice("HLDEMO\x00\x00".as_bytes());

        self.write_header(demo.header);
        self.write_directory(demo.directory);
    }

    fn write_header(&mut self, header: Header) {
        self.writer.append_i32(header.demo_protocol);
        self.writer.append_i32(header.net_protocol);
        self.writer.append_u8_slice(header.map_name);
        self.writer.append_u8_slice(header.game_dir);
        self.writer.append_u32(header.map_crc);
    }

    fn write_directory(&mut self, directory: Directory) {
        // Delay writing directory offset
        let directory_offset_pos = self.writer.get_offset();
        self.writer.append_i32(0i32);

        let mut entry_offsets: Vec<usize> = Vec::new();

        for entry in &directory.entries {
            let mut has_written_next_section = false;

            entry_offsets.push(self.writer.get_offset());

            for frame in &entry.frames {
                self.write_frame(frame);

                if matches!(frame.data, FrameData::NextSection) {
                    has_written_next_section = true;
                }
            }

            if !has_written_next_section {
                self.writer.append_u8(5u8);
                self.writer.append_f32(0.);
                self.writer.append_i32(0);
            }
        }

        let director_offset = self.writer.get_offset();
        self.writer.append_i32(directory.entries.len() as i32);

        for (entry, offset) in directory.entries.iter().zip(entry_offsets.iter()) {
            self.write_directory_entry(entry, offset);
        }

        self.writer.data.splice(
            directory_offset_pos..directory_offset_pos + 4,
            (director_offset as u32).to_le_bytes(),
        );
    }

    fn write_directory_entry(&mut self, entry: &DirectoryEntry, new_offset: &usize) {
        self.writer.append_i32(entry.entry_type);
        self.writer.append_u8_slice(entry.description);
        self.writer.append_i32(entry.flags);
        self.writer.append_i32(entry.cd_track);
        self.writer.append_f32(entry.track_time);
        self.writer.append_i32(entry.frame_count);
        self.writer.append_i32(*new_offset as i32);
        self.writer.append_i32(entry.file_length);
    }

    fn write_frame(&mut self, frame: &Frame) {
        match &frame.data {
            FrameData::DemoStart => self.writer.append_u8(2u8),
            FrameData::ConsoleCommand(_) => self.writer.append_u8(3u8),
            FrameData::ClientData(_) => self.writer.append_u8(4u8),
            FrameData::NextSection => self.writer.append_u8(5u8),
            FrameData::Event(_) => self.writer.append_u8(6u8),
            FrameData::WeaponAnim(_) => self.writer.append_u8(7u8),
            FrameData::Sound(_) => self.writer.append_u8(8u8),
            FrameData::DemoBuffer(_) => self.writer.append_u8(9u8),
            FrameData::NetMsg((type_, _)) => match type_ {
                NetMsgFrameType::Start => self.writer.append_u8(0u8),
                NetMsgFrameType::Normal => self.writer.append_u8(1u8),
                NetMsgFrameType::Unknown(what) => self.writer.append_u8(*what),
            },
        }

        self.writer.append_f32(frame.time);
        self.writer.append_i32(frame.frame);
        self.write_frame_data(&frame.data);
    }
    fn write_frame_data(&mut self, frame: &FrameData) {
        match frame {
            FrameData::DemoStart => (),
            FrameData::ConsoleCommand(frame) => self.writer.append_u8_slice(frame.command),
            FrameData::ClientData(frame) => {
                self.writer.append_f32_array(frame.origin);
                self.writer.append_f32_array(frame.viewangles);
                self.writer.append_i32(frame.weapon_bits);
                self.writer.append_f32(frame.fov);
            }
            FrameData::NextSection => (),
            FrameData::Event(frame) => {
                self.writer.append_i32(frame.flags);
                self.writer.append_i32(frame.index);
                self.writer.append_f32(frame.delay);

                self.writer.append_i32(frame.args.flags);
                self.writer.append_i32(frame.args.entity_index);
                self.writer.append_f32_array(frame.args.origin);
                self.writer.append_f32_array(frame.args.angles);
                self.writer.append_f32_array(frame.args.velocity);
                self.writer.append_i32(frame.args.ducking);
                self.writer.append_f32(frame.args.fparam1);
                self.writer.append_f32(frame.args.fparam2);
                self.writer.append_i32(frame.args.iparam1);
                self.writer.append_i32(frame.args.iparam2);
                self.writer.append_i32(frame.args.bparam1);
                self.writer.append_i32(frame.args.bparam2);
            }
            FrameData::WeaponAnim(frame) => {
                self.writer.append_i32(frame.anim);
                self.writer.append_i32(frame.body);
            }
            FrameData::Sound(frame) => {
                self.writer.append_i32(frame.channel);
                self.writer.append_i32(frame.sample.len() as i32);
                self.writer.append_u8_slice(frame.sample);
                self.writer.append_f32(frame.attenuation);
                self.writer.append_f32(frame.volume);
                self.writer.append_i32(frame.flags);
                self.writer.append_i32(frame.pitch);
            }
            FrameData::DemoBuffer(frame) => {
                self.writer.append_i32(frame.buffer.len() as i32);
                self.writer.append_u8_slice(frame.buffer);
            }
            FrameData::NetMsg((_type_, data)) => {
                self.writer.append_f32(data.info.timestamp);
                // ref_params
                self.writer.append_f32_array(data.info.ref_params.vieworg);
                self.writer
                    .append_f32_array(data.info.ref_params.viewangles);
                self.writer.append_f32_array(data.info.ref_params.forward);
                self.writer.append_f32_array(data.info.ref_params.right);
                self.writer.append_f32_array(data.info.ref_params.up);
                self.writer.append_f32(data.info.ref_params.frametime);
                self.writer.append_f32(data.info.ref_params.time);
                self.writer.append_i32(data.info.ref_params.intermission);
                self.writer.append_i32(data.info.ref_params.paused);
                self.writer.append_i32(data.info.ref_params.spectator);
                self.writer.append_i32(data.info.ref_params.onground);
                self.writer.append_i32(data.info.ref_params.waterlevel);
                self.writer.append_f32_array(data.info.ref_params.simvel);
                self.writer.append_f32_array(data.info.ref_params.simorg);
                self.writer
                    .append_f32_array(data.info.ref_params.viewheight);
                self.writer.append_f32(data.info.ref_params.idealpitch);
                self.writer
                    .append_f32_array(data.info.ref_params.cl_viewangles);
                self.writer.append_i32(data.info.ref_params.health);
                self.writer
                    .append_f32_array(data.info.ref_params.crosshairangle);
                self.writer.append_f32(data.info.ref_params.viewsize);
                self.writer
                    .append_f32_array(data.info.ref_params.punchangle);
                self.writer.append_i32(data.info.ref_params.maxclients);
                self.writer.append_i32(data.info.ref_params.viewentity);
                self.writer.append_i32(data.info.ref_params.playernum);
                self.writer.append_i32(data.info.ref_params.max_entities);
                self.writer.append_i32(data.info.ref_params.demoplayback);
                self.writer.append_i32(data.info.ref_params.hardware);
                self.writer.append_i32(data.info.ref_params.smoothing);
                self.writer.append_i32(data.info.ref_params.ptr_cmd);
                self.writer.append_i32(data.info.ref_params.ptr_movevars);
                self.writer
                    .append_i32_array_4(data.info.ref_params.viewport);
                self.writer.append_i32(data.info.ref_params.next_view);
                self.writer
                    .append_i32(data.info.ref_params.only_client_draw);
                // usercmd
                self.writer.append_i16(data.info.usercmd.lerp_msec);
                self.writer.append_u8(data.info.usercmd.msec);
                self.writer.append_u8(0u8); // unknown
                self.writer.append_f32_array(data.info.usercmd.viewangles);
                self.writer.append_f32(data.info.usercmd.forwardmove);
                self.writer.append_f32(data.info.usercmd.sidemove);
                self.writer.append_f32(data.info.usercmd.upmove);
                self.writer.append_i8(data.info.usercmd.lightlevel);
                self.writer.append_u8(0u8); // unknown
                self.writer.append_u16(data.info.usercmd.buttons);
                self.writer.append_i8(data.info.usercmd.impulse);
                self.writer.append_i8(data.info.usercmd.weaponselect);
                self.writer.append_u8(0u8); // unknown
                self.writer.append_u8(0u8); // unknown
                self.writer.append_i32(data.info.usercmd.impact_index);
                self.writer
                    .append_f32_array(data.info.usercmd.impact_position);
                // movevars
                self.writer.append_f32(data.info.movevars.gravity);
                self.writer.append_f32(data.info.movevars.stopspeed);
                self.writer.append_f32(data.info.movevars.maxspeed);
                self.writer.append_f32(data.info.movevars.spectatormaxspeed);
                self.writer.append_f32(data.info.movevars.accelerate);
                self.writer.append_f32(data.info.movevars.airaccelerate);
                self.writer.append_f32(data.info.movevars.wateraccelerate);
                self.writer.append_f32(data.info.movevars.friction);
                self.writer.append_f32(data.info.movevars.edgefriction);
                self.writer.append_f32(data.info.movevars.waterfriction);
                self.writer.append_f32(data.info.movevars.entgravity);
                self.writer.append_f32(data.info.movevars.bounce);
                self.writer.append_f32(data.info.movevars.stepsize);
                self.writer.append_f32(data.info.movevars.maxvelocity);
                self.writer.append_f32(data.info.movevars.zmax);
                self.writer.append_f32(data.info.movevars.wave_height);
                self.writer.append_i32(data.info.movevars.footsteps);
                self.writer.append_u8_slice(data.info.movevars.sky_name);
                self.writer.append_f32(data.info.movevars.rollangle);
                self.writer.append_f32(data.info.movevars.rollspeed);
                self.writer.append_f32(data.info.movevars.skycolor_r);
                self.writer.append_f32(data.info.movevars.skycolor_g);
                self.writer.append_f32(data.info.movevars.skycolor_b);
                self.writer.append_f32(data.info.movevars.skyvec_x);
                self.writer.append_f32(data.info.movevars.skyvec_y);
                self.writer.append_f32(data.info.movevars.skyvec_z);
                // still in info
                self.writer.append_f32_array(data.info.view);
                self.writer.append_i32(data.info.viewmodel);
                // now other data
                self.writer.append_i32(data.incoming_sequence);
                self.writer.append_i32(data.incoming_acknowledged);
                self.writer.append_i32(data.incoming_reliable_acknowledged);
                self.writer.append_i32(data.incoming_reliable_sequence);
                self.writer.append_i32(data.outgoing_sequence);
                self.writer.append_i32(data.reliable_sequence);
                self.writer.append_i32(data.last_reliable_sequence);

                self.writer.append_i32(data.msg.len() as i32);
                self.writer.append_u8_slice(data.msg);
            }
        }
    }
}
