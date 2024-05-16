use super::*;

impl Doer for SvcTimeScale {
    fn id(&self) -> u8 {
        55
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(le_f32, |time_scale| SvcTimeScale { time_scale })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_f32(self.time_scale);

        writer.data
    }
}
