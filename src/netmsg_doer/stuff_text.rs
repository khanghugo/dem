use super::*;

impl Doer for SvcStuffText {
    fn id(&self) -> u8 {
        9
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(null_string, |command| SvcStuffText {
            command: command.into(),
        })(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(self.command.as_slice());

        writer.data
    }
}
