use super::*;

impl Doer for SvcSignOnNum {
    fn id(&self) -> u8 {
        25
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(le_i8, |sign| SvcSignOnNum { sign }).parse(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i8(self.sign);

        writer.data
    }
}
