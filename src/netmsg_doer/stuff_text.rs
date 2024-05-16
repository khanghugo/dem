use super::*;

impl Doer for SvcStuffText {
    fn id(&self) -> u8 {
        9
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(null_string, |command| SvcStuffText {
            command: command.to_vec(),
        })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.command);

        writer.data
    }
}
