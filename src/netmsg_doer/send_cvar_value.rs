use super::*;

impl Doer for SvcSendCvarValue {
    fn id(&self) -> u8 {
        57
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(null_string, |name| SvcSendCvarValue { name: name.into() }).parse(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(self.name.as_slice());

        writer.data
    }
}
