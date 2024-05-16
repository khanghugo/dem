use super::*;

impl Doer for SvcDirector {
    fn id(&self) -> u8 {
        51
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        let (i, (length, flag)) = tuple((le_u8, le_u8))(i)?;
        let (i, message) = take(length - 1)(i)?;

        Ok((
            i,
            SvcDirector {
                length,
                flag,
                message: message.to_vec(),
            },
        ))
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.length);
        writer.append_u8(self.flag);
        writer.append_u8_slice(&self.message);

        writer.data
    }
}
