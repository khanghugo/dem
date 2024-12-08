use super::*;

impl Doer for SvcTime {
    fn id(&self) -> u8 {
        7
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(le_f32, |time| SvcTime { time })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_f32(self.time);

        writer.data
    }
}
