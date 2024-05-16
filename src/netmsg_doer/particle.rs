use super::*;

impl Doer for SvcParticle {
    fn id(&self) -> u8 {
        18
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(
            tuple((count(le_i16, 3), take(3usize), le_u8, le_u8)),
            |(origin, direction, count, color): (Vec<i16>, &[u8], _, _)| SvcParticle {
                origin,
                direction: direction.to_vec(),
                count,
                color,
            },
        )(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        for j in 0..3 {
            writer.append_i16(self.origin[j])
        }
        writer.append_u8_slice(&self.direction);
        writer.append_u8(self.count);
        writer.append_u8(self.color);

        writer.data
    }
}
