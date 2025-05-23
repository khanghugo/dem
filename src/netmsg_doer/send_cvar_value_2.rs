use super::*;

impl Doer for SvcSendCvarValue2 {
    fn id(&self) -> u8 {
        58
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(tuple((le_u32, null_string)), |(request_id, name)| {
            SvcSendCvarValue2 {
                request_id,
                name: name.into(),
            }
        })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u32(self.request_id);
        writer.append_u8_slice(self.name.as_slice());

        writer.data
    }
}
