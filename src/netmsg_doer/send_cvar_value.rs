use super::*;

impl Doer for SvcSendCvarValue {
    fn id(&self) -> u8 {
        57
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(null_string, |name| SvcSendCvarValue {
            name: name.to_vec(),
        })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.name);

        writer.data
    }
}
