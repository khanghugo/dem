use super::*;

impl Doer for SvcSpawnStaticSound {
    fn id(&self) -> u8 {
        29
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(
            tuple((count(le_i16, 3), le_u16, le_u8, le_u8, le_u16, le_u8, le_u8)),
            |(origin, sound_index, volume, attenuation, entity_index, pitch, flags)| {
                SvcSpawnStaticSound {
                    origin,
                    sound_index,
                    volume,
                    attenuation,
                    entity_index,
                    pitch,
                    flags,
                }
            },
        )(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        for what in &self.origin {
            writer.append_i16(*what)
        }
        writer.append_u16(self.sound_index);
        writer.append_u8(self.volume);
        writer.append_u8(self.attenuation);
        writer.append_u16(self.entity_index);
        writer.append_u8(self.pitch);
        writer.append_u8(self.flags);

        writer.data
    }
}
