use super::*;

impl Doer for SvcWeaponAnim {
    fn id(&self) -> u8 {
        35
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(
            tuple((le_i8, le_i8)),
            |(sequence_number, weapon_model_body_group)| SvcWeaponAnim {
                sequence_number,
                weapon_model_body_group,
            },
        )(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i8(self.sequence_number);
        writer.append_i8(self.weapon_model_body_group);

        writer.data
    }
}
