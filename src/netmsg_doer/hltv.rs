use super::*;

impl Doer for SvcHltv {
    fn id(&self) -> u8 {
        50
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(le_u8, |mode| SvcHltv { mode })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.mode);

        writer.data
    }
}
