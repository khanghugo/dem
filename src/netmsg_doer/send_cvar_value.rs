use super::*;

impl Doer for SvcSendCvarValue {
    fn id(&self) -> u8 {
        57
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(null_string, |name| SvcSendCvarValue {
            name: name.to_vec(),
        })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.name);

        writer.data
    }
}
