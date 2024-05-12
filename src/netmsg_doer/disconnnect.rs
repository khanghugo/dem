use super::*;

impl Doer<SvcDisconnect> for SvcDisconnect {
    fn id(&self) -> u8 {
        2
    }

    fn parse(i: &[u8], _: Aux) -> Result<SvcDisconnect> {
        map(null_string, |reason| SvcDisconnect {
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
