use super::*;

impl Doer for SvcSendCvarValue {
    fn id(&self) -> u8 {
        57
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(null_string, |name| SvcSendCvarValue { name: name.into() })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(self.name.as_slice());

        writer.data
    }
}
