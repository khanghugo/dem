use super::*;

impl Doer for SvcAddAngle {
    fn id(&self) -> u8 {
        38
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(le_i16, |angle_to_add| Self { angle_to_add })(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i16(self.angle_to_add);

        writer.data
    }
}
