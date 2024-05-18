use super::*;

impl Doer for SvcHltv {
    fn id(&self) -> u8 {
        50
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(le_u8, |mode| SvcHltv { mode })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.mode);

        writer.data
    }
}
