use super::*;

impl Doer for SvcVoiceData {
    fn id(&self) -> u8 {
        53
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        let (i, (player_index, size)) = tuple((le_u8, le_u16))(i)?;
        let (i, data) = take(size)(i)?;

        Ok((
            i,
            SvcVoiceData {
                player_index,
                size,
                data: data.to_vec(),
            },
        ))
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.player_index);
        writer.append_u16(self.size);
        writer.append_u8_slice(&self.data);

        writer.data
    }
}
