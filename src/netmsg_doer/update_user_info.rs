use super::*;

impl Doer for SvcUpdateUserInfo {
    fn id(&self) -> u8 {
        13
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(
            (le_u8, le_u32, null_string, take(16usize)),
            |(index, id, user_info, cd_key_hash)| SvcUpdateUserInfo {
                index,
                id,
                user_info: user_info.into(),
                cd_key_hash: cd_key_hash.into(),
            },
        )
        .parse(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.index);
        writer.append_u32(self.id);
        writer.append_u8_slice(self.user_info.as_slice());
        writer.append_u8_slice(self.cd_key_hash.padded(16).as_slice());

        writer.data
    }
}
