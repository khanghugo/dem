use super::*;

impl Doer for SvcSendCvarValue2 {
    fn id(&self) -> u8 {
        58
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(tuple((le_u32, null_string)), |(request_id, name)| {
            SvcSendCvarValue2 {
                request_id,
                name: name.to_vec(),
            }
        })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u32(self.request_id);
        writer.append_u8_slice(&self.name);

        writer.data
    }
}
