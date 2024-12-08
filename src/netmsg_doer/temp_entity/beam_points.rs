use super::*;

impl Doer for TeBeamPoints {
    fn id(&self) -> u8 {
        0
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(
            tuple((
                count(le_i16, 3),
                count(le_i16, 3),
                le_i16,
                le_u8,
                le_u8,
                le_u8,
                le_u8,
                le_u8,
                take(4usize),
                le_u8,
            )),
            |(
                start_position,
                end_position,
                sprite_index,
                start_frame,
                frame_rate,
                life,
                width,
                noise,
                color,
                speed,
            ): (_, _, _, _, _, _, _, _, &[u8], _)| {
                Self {
                    start_position,
                    end_position,
                    sprite_index,
                    start_frame,
                    frame_rate,
                    life,
                    width,
                    noise,
                    color: color.to_vec(),
                    speed,
                }
            },
        )(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_i16_slice(self.start_position.as_slice());
        writer.append_i16_slice(self.end_position.as_slice());
        writer.append_i16(self.sprite_index);
        writer.append_u8(self.start_frame);
        writer.append_u8(self.frame_rate);
        writer.append_u8(self.life);
        writer.append_u8(self.width);
        writer.append_u8(self.noise);
        writer.append_u8_slice(&self.color);
        writer.append_u8(self.speed);

        writer.data
    }
}
