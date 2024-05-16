use super::*;

impl Doer for SvcDisconnect {
    fn id(&self) -> u8 {
        2
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(null_string, |reason| Self {
            reason: reason.to_vec(),
        })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.reason);

        writer.data
    }
}
