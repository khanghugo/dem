use super::*;

impl Doer for TeBspDecal {
    fn id(&self) -> u8 {
        13
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        let (i, unknown1) = take(8usize).parse(i)?;
        let (i, entity_index) = le_i16(i)?;
        let (i, unknown2) = if entity_index != 0 {
            map(take(2usize), |i: &[u8]| Some(i.to_owned())).parse(i)?
        } else {
            (i, None)
        };

        Ok((
            i,
            Self {
                unknown1: unknown1.to_owned(),
                entity_index,
                unknown2,
            },
        ))
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8_slice(&self.unknown1);
        writer.append_i16(self.entity_index);
        if self.entity_index != 0 {
            writer.append_u8_slice(self.unknown2.as_ref().unwrap());
        }

        writer.data
    }
}
