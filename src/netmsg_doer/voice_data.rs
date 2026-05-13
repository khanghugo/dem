use super::*;

impl Doer for SvcVoiceData {
    fn id(&self) -> u8 {
        53
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        let (i, (player_index, size)) = (le_u8, le_u16).parse(i)?;
        let (i, data) = take(size).parse(i)?;

        Ok((
            i,
            SvcVoiceData {
                player_index,
                size,
                data: data.to_owned(),
            },
        ))
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.player_index);
        writer.append_u16(self.size);
        writer.append_u8_slice(&self.data);

        writer.data
    }
}
