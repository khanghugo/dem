use super::*;

impl Doer for SvcResourceLocation {
    fn id(&self) -> u8 {
        56
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(null_string, |download_url| SvcResourceLocation {
            download_url: download_url.to_vec(),
        })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.download_url);

        writer.data
    }
}
