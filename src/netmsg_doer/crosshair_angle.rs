use super::*;

impl Doer for SvcCrosshairAngle {
    fn id(&self) -> u8 {
        47
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(tuple((le_i16, le_i16)), |(pitch, yaw)| Self { pitch, yaw })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i16(self.pitch);
        writer.append_i16(self.yaw);

        writer.data
    }
}
