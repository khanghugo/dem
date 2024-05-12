use super::*;

impl Doer<SvcCutscene> for SvcCutscene {
    fn id(&self) -> u8 {
        34
    }

    fn parse(i: &[u8], _: Aux) -> Result<SvcCutscene> {
        map(null_string, |text| SvcCutscene {
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
