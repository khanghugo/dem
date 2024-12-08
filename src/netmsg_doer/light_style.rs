use super::*;

impl Doer for SvcLightStyle {
    fn id(&self) -> u8 {
        12
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(tuple((le_u8, null_string)), |(index, light_info)| {
            SvcLightStyle {
                index,
                light_info: light_info.to_vec(),
            }
        })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.index);
        writer.append_u8_slice(&self.light_info);

        writer.data
    }
}
