use super::*;

impl Doer for SvcDisconnect {
    fn id(&self) -> u8 {
        2
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(null_string, |reason| Self {
            reason: reason.to_owned(),
        })(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.reason);

        writer.data
    }
}
