use super::*;

impl Doer for SvcFinale {
    fn id(&self) -> u8 {
        31
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(null_string, |text| SvcFinale {
            text: text.to_vec(),
        })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.text);

        writer.data
    }
}
