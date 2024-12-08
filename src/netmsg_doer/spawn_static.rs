use super::*;

impl Doer for SvcSpawnStatic {
    fn id(&self) -> u8 {
        20
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        let (
            i,
            (
                model_index,
                sequence,
                frame,
                color_map,
                skin,
                origin_x,
                rotation_x,
                origin_y,
                rotation_y,
                origin_z,
                rotation_z,
                has_render_mode,
            ),
        ) = tuple((
            le_i16, le_i8, le_i8, le_i16, le_i8, le_i16, le_i8, le_i16, le_i8, le_i16, le_i8, le_i8,
        ))(i)?;

        let (i, render_color) = if has_render_mode != 0 {
            map(take(3usize), |what: &[u8]| Some(what.to_vec()))(i)?
        } else {
            (i, None)
        };

        Ok((
            i,
            SvcSpawnStatic {
                model_index,
                sequence,
                frame,
                color_map,
                skin,
                origin_x,
                rotation_x,
                origin_y,
                rotation_y,
                origin_z,
                rotation_z,
                has_render_mode,
                render_color,
            },
        ))
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i16(self.model_index);
        writer.append_i8(self.sequence);
        writer.append_i8(self.frame);
        writer.append_i16(self.color_map);
        writer.append_i8(self.skin);
        writer.append_i16(self.origin_x);
        writer.append_i8(self.rotation_x);
        writer.append_i16(self.origin_y);
        writer.append_i8(self.rotation_y);
        writer.append_i16(self.origin_z);
        writer.append_i8(self.rotation_z);
        writer.append_i8(self.has_render_mode);

        if self.has_render_mode != 0 {
            writer.append_u8_slice(self.render_color.as_ref().unwrap());
        }

        writer.data
    }
}
