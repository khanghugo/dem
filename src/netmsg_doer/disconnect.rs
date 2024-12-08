use super::*;

impl Doer for SvcDisconnect {
    fn id(&self) -> u8 {
        2
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(null_string, |reason| Self {
            reason: reason.to_vec(),
        })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.reason);

        writer.data
    }
}
