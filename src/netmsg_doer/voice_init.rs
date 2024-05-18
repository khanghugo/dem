use super::*;

impl Doer for SvcVoiceInit {
    fn id(&self) -> u8 {
        52
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(tuple((null_string, le_i8)), |(codec_name, quality)| {
            SvcVoiceInit {
                codec_name: codec_name.to_vec(),
                quality,
            }
        })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.codec_name);
        writer.append_i8(self.quality);

        writer.data
    }
}
