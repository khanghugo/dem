use super::*;

impl Doer for SvcVersion {
    fn id(&self) -> u8 {
        4
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(le_u32, |protocol_version| SvcVersion { protocol_version })(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u32(self.protocol_version);

        writer.data
    }
}
