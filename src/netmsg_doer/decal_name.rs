use super::*;

impl Doer for SvcDecalName {
    fn id(&self) -> u8 {
        36
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(
            tuple((le_u8, null_string)),
            |(position_index, decal_name)| Self {
                position_index,
                decal_name: decal_name.to_vec(),
            },
        )(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.position_index);
        writer.append_u8_slice(&self.decal_name);

        writer.data
    }
}
