use super::*;

impl Doer for SvcUpdateUserInfo {
    fn id(&self) -> u8 {
        13
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(
            tuple((le_u8, le_u32, null_string, take(16usize))),
            |(index, id, user_info, cd_key_hash)| SvcUpdateUserInfo {
                index,
                id,
                user_info: user_info.to_vec(),
                cd_key_hash: cd_key_hash.to_vec(),
            },
        )(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.index);
        writer.append_u32(self.id);
        writer.append_u8_slice(&self.user_info);
        writer.append_u8_slice(&self.cd_key_hash);

        writer.data
    }
}
