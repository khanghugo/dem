use super::*;

impl Doer<SvcAddAngle> for SvcAddAngle {
    fn id(&self) -> u8 {
        38
    }

    fn parse(i: &[u8], _: Aux) -> Result<SvcAddAngle> {
        map(le_i16, |angle_to_add| SvcAddAngle { angle_to_add })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i16(self.angle_to_add);

        writer.data
    }
}
