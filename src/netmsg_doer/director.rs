use super::*;

impl Doer for SvcDirector {
    fn id(&self) -> u8 {
        51
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        // https://github.com/ValveSoftware/halflife/blob/b1b5cf5892918535619b2937bb927e46cb097ba1/common/hltv.h#L17-L35
        let (i, (length, command)) = (le_u8, le_u8).parse(i)?;
        let (i, message) = take(length - 1).parse(i)?;

        Ok((
            i,
            SvcDirector {
                length,
                command,
                message: message.into(),
            },
        ))
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.length);
        writer.append_u8(self.command);
        writer.append_u8_slice(self.message.as_slice());

        writer.data
    }
}
