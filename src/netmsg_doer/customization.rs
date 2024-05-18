use super::*;

impl Doer for SvcCustomization {
    fn id(&self) -> u8 {
        46
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        let (i, (player_index, type_, name, index, download_size, flags)) =
            tuple((le_u8, le_u8, null_string, le_u16, le_u32, le_u8))(i)?;

        let (i, md5_hash) = if flags & 4 != 0 {
            map(take(16usize), |what: &[u8]| Some(what.to_owned()))(i)?
        } else {
            (i, None)
        };

        Ok((
            i,
            Self {
                player_index,
                type_,
                name: name.to_owned(),
                index,
                download_size,
                flags,
                md5_hash,
            },
        ))
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.player_index);
        writer.append_u8(self.type_);
        writer.append_u8_slice(&self.name);
        writer.append_u16(self.index);
        writer.append_u32(self.download_size);
        writer.append_u8(self.flags);

        if self.flags & 4 != 0 {
            writer.append_u8_slice(self.md5_hash.as_ref().unwrap());
        }

        writer.data
    }
}
