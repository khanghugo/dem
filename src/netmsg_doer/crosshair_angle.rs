use super::*;

impl Doer for SvcCrosshairAngle {
    fn id(&self) -> u8 {
        47
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(tuple((le_i16, le_i16)), |(pitch, yaw)| Self {
            pitch,
            yaw,
        })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i16(self.pitch);
        writer.append_i16(self.yaw);

        writer.data
    }
}
