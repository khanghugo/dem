use super::*;

impl Doer for SvcTime {
    fn id(&self) -> u8 {
        7
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(le_f32, |time| SvcTime { time })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_f32(self.time);

        writer.data
    }
}
