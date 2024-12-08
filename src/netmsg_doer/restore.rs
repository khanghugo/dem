use super::*;

impl Doer for SvcRestore {
    fn id(&self) -> u8 {
        33
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        let (i, (save_name, map_count)) = tuple((null_string, le_u8))(i)?;
        let (i, map_names) = count(map(null_string, |s| s.to_vec()), map_count as usize)(i)?;

        Ok((
            i,
            SvcRestore {
                save_name: save_name.to_vec(),
                map_count,
                map_names,
            },
        ))
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
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
