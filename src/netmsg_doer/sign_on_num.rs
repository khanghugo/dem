use super::*;

impl Doer for SvcSignOnNum {
    fn id(&self) -> u8 {
        25
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(le_i8, |sign| SvcSignOnNum { sign })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i8(self.sign);

        writer.data
    }
}
