use super::*;

impl Doer for TeTextMessage {
    fn id(&self) -> u8 {
        29
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        let (
            i,
            (
                channel,
                x,
                y,
                effect,
                text_color,
                effect_color,
                fade_in_time,
                fade_out_time,
                hold_time,
            ),
        ) = (
            le_i8,
            le_i16,
            le_i16,
            le_i8,
            take(4usize),
            take(4usize),
            le_i16,
            le_i16,
            le_i16,
        )
            .parse(i)?;

        let (i, effect_time) = if effect != 0 {
            map(le_i16, Some).parse(i)?
        } else {
            (i, None)
        };

        let (i, message) = null_string(i)?;

        Ok((
            i,
            Self {
                channel,
                x,
                y,
                effect,
                text_color: text_color.to_owned(),
                effect_color: effect_color.to_owned(),
                fade_in_time,
                fade_out_time,
                hold_time,
                effect_time,
                message: message.into(),
            },
        ))
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_i8(self.channel);
        writer.append_i16(self.x);
        writer.append_i16(self.y);
        writer.append_i8(self.effect);
        writer.append_u8_slice(&self.text_color);
        writer.append_u8_slice(&self.effect_color);
        writer.append_i16(self.fade_in_time);
        writer.append_i16(self.fade_out_time);
        writer.append_i16(self.hold_time);

        if self.effect != 0 {
            writer.append_i16(self.effect_time.unwrap());
        }

        writer.append_u8_slice(self.message.as_slice());

        writer.data
    }
}
