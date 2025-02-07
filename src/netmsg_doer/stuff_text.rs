use super::*;

impl Doer for SvcStuffText {
    fn id(&self) -> u8 {
        9
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(null_string, |command| SvcStuffText {
            command: command.into(),
        })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(self.command.as_slice());

        writer.data
    }
}
