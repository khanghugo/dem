use super::*;

impl Doer for SvcVoiceInit {
    fn id(&self) -> u8 {
        52
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(tuple((null_string, le_i8)), |(codec_name, quality)| {
            SvcVoiceInit {
                codec_name: codec_name.to_vec(),
                quality,
            }
        })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.codec_name);
        writer.append_i8(self.quality);

        writer.data
    }
}
