use super::*;

impl Doer for SvcResourceRequest {
    fn id(&self) -> u8 {
        45
    }

    fn parse<'a>(i: &'a [u8], _: &'a RefCell<Aux>) -> Result<'a, Self> {
        map(
            tuple((le_i32, count(le_u8, 4usize))),
            |(spawn_count, unknown)| SvcResourceRequest {
                spawn_count,
                unknown,
            },
        )(i)
    }

    fn write(&self, _: &RefCell<Aux>) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i32(self.spawn_count);

        for what in &self.unknown {
            writer.append_u8(*what);
        }
        writer.data
    }
}
