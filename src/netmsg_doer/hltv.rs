use super::*;

impl Doer for SvcHltv {
    fn id(&self) -> u8 {
        50
    }

    fn parse<'a>(i: &'a [u8], aux: &mut DemoGlobalState) -> NomResult<'a, Self> {
        aux.is_hltv = true;

        map(le_u8, |mode| SvcHltv { mode }).parse(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.mode);

        writer.data
    }
}
