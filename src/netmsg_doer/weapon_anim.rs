use super::*;

impl Doer for SvcWeaponAnim {
    fn id(&self) -> u8 {
        35
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(
            (le_i8, le_i8),
            |(sequence_number, weapon_model_body_group)| SvcWeaponAnim {
                sequence_number,
                weapon_model_body_group,
            },
        )
        .parse(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i8(self.sequence_number);
        writer.append_i8(self.weapon_model_body_group);

        writer.data
    }
}
