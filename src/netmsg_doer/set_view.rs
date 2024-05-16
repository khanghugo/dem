use super::*;

impl Doer for SvcSetView {
    fn id(&self) -> u8 {
        5
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(le_i16, |entity_index| SvcSetView { entity_index })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i16(self.entity_index);

        writer.data
    }
}
