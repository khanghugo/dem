use super::*;

impl Doer for SvcStopSound {
    fn id(&self) -> u8 {
        16
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(le_i16, |entity_index| SvcStopSound { entity_index })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i16(self.entity_index);

        writer.data
    }
}
