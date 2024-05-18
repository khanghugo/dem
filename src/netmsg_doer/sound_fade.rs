use super::*;

impl Doer for SvcSoundFade {
    fn id(&self) -> u8 {
        48
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(
            tuple((le_u8, le_u8, le_u8, le_u8)),
            |(initial_percent, hold_time, fade_out_time, fade_in_time)| SvcSoundFade {
                initial_percent,
                hold_time,
                fade_out_time,
                fade_in_time,
            },
        )(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.initial_percent);
        writer.append_u8(self.hold_time);
        writer.append_u8(self.fade_in_time);
        writer.append_u8(self.fade_out_time);

        writer.data
    }
}
