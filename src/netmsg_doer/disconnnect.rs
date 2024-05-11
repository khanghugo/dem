use crate::types::SvcDisconnect;

use super::*;

impl Doer<SvcDisconnect> for SvcDisconnect {
    fn get_id(&self) -> u8 {
        2
    }

    fn parse(i: &[u8], _: Option<Aux>) -> Result<SvcDisconnect> {
        map(null_string, |reason| SvcDisconnect {
            reason: reason.to_vec(),
        })(i)
    }

    fn write(&self, _: Option<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.get_id());

        writer.append_u8_slice(&self.reason);

        writer.data
    }
}
