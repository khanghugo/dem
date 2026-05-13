use super::*;

impl Doer for SvcStopSound {
    fn id(&self) -> u8 {
        16
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(le_i16, |entity_index| SvcStopSound { entity_index }).parse(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i16(self.entity_index);

        writer.data
    }
}
