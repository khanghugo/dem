use super::*;

impl Doer for SvcSendExtraInfo {
    fn id(&self) -> u8 {
        54
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(tuple((null_string, le_u8)), |(fallback_dir, can_cheat)| {
            SvcSendExtraInfo {
                fallback_dir: fallback_dir.to_vec(),
                can_cheat,
            }
        })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.fallback_dir);
        writer.append_u8(self.can_cheat);

        writer.data
    }
}
