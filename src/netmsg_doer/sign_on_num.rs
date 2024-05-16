use super::*;

impl Doer for SvcSignOnNum {
    fn id(&self) -> u8 {
        25
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(le_i8, |sign| SvcSignOnNum { sign })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i8(self.sign);

        writer.data
    }
}
