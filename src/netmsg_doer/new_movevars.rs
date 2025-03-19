use super::*;

impl Doer for SvcNewMovevars {
    fn id(&self) -> u8 {
        44
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        // https://github.com/rust-bakery/nom/issues/1144
        map(
            tuple((
                tuple((
                    le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32,
                    le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_u8, le_f32, le_f32,
                )),
                count(le_f32, 3),
                count(le_f32, 3),
                null_string,
            )),
            |
            (
                (gravity,
                stop_speed,
                max_speed,
                spectator_max_speed,
                accelerate,
                airaccelerate,
                water_accelerate,
                friction,
                edge_friction,
                water_friction,
                ent_garvity,
                bounce,
                step_size,
                max_velocity,
                z_max,
                wave_height,
                footsteps,
                roll_angle,
                roll_speed),
                sky_color,
                sky_vec,
                sky_name,
            )
            // what
            | SvcNewMovevars {
                gravity,
                stop_speed,
                max_speed,
                spectator_max_speed,
                accelerate,
                airaccelerate,
                water_accelerate,
                friction,
                edge_friction,
                water_friction,
                ent_garvity,
                bounce,
                step_size,
                max_velocity,
                z_max,
                wave_height,
                footsteps,
                roll_angle,
                roll_speed,
                sky_color,
                sky_vec,
                sky_name: sky_name.to_vec(),
            },
        )(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_f32(self.gravity);
        writer.append_f32(self.stop_speed);
        writer.append_f32(self.max_speed);
        writer.append_f32(self.spectator_max_speed);
        writer.append_f32(self.accelerate);
        writer.append_f32(self.airaccelerate);
        writer.append_f32(self.water_accelerate);
        writer.append_f32(self.friction);
        writer.append_f32(self.edge_friction);
        writer.append_f32(self.water_friction);
        writer.append_f32(self.ent_garvity);
        writer.append_f32(self.bounce);
        writer.append_f32(self.step_size);
        writer.append_f32(self.max_velocity);
        writer.append_f32(self.z_max);
        writer.append_f32(self.wave_height);
        writer.append_u8(self.footsteps);
        writer.append_f32(self.roll_angle);
        writer.append_f32(self.roll_speed);
        for e in &self.sky_color {
            writer.append_f32(*e);
        }
        for e in &self.sky_vec {
            writer.append_f32(*e);
        }
        writer.append_u8_slice(&self.sky_name);

        writer.data
    }
}
