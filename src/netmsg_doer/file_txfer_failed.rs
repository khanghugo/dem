use super::*;

impl Doer for SvcFileTxferFailed {
    fn id(&self) -> u8 {
        49
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(null_string, |file_name| SvcFileTxferFailed {
            file_name: file_name.to_vec(),
        })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.file_name);

        writer.data
    }
}
