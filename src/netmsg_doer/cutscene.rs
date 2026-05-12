use super::*;

impl Doer for SvcCutscene {
    fn id(&self) -> u8 {
        34
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(null_string, |text| Self {
            text: text.to_vec(),
        })(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.text);

        writer.data
    }
}
