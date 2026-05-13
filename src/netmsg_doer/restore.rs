use super::*;

impl Doer for SvcRestore {
    fn id(&self) -> u8 {
        33
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        let (i, (save_name, map_count)) = (null_string, le_u8).parse(i)?;
        let (i, map_names) =
            count(map(null_string, |s| s.to_owned()), map_count as usize).parse(i)?;

        Ok((
            i,
            SvcRestore {
                save_name: save_name.to_owned(),
                map_count,
                map_names,
            },
        ))
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.save_name);
        writer.append_u8(self.map_count);
        for what in &self.map_names {
            writer.append_u8_slice(what);
        }

        writer.data
    }
}
