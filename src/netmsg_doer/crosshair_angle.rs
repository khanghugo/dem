use super::*;

impl Doer<SvcCrosshairAngle> for SvcCrosshairAngle {
    fn id(&self) -> u8 {
        47
    }

    fn parse(i: &[u8], _: Aux) -> Result<SvcCrosshairAngle> {
        map(tuple((le_i16, le_i16)), |(pitch, yaw)| SvcCrosshairAngle {
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
