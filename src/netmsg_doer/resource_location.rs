use super::*;

impl Doer for SvcResourceLocation {
    fn id(&self) -> u8 {
        56
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(null_string, |download_url| SvcResourceLocation {
            download_url: download_url.to_vec(),
        })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.download_url);

        writer.data
    }
}
