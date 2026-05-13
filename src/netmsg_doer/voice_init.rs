use super::*;

impl Doer for SvcVoiceInit {
    fn id(&self) -> u8 {
        52
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map((null_string, le_i8), |(codec_name, quality)| SvcVoiceInit {
            codec_name: codec_name.to_owned(),
            quality,
        })
        .parse(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.codec_name);
        writer.append_i8(self.quality);

        writer.data
    }
}
