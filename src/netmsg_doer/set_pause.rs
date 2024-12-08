use super::*;

impl Doer for SvcSetPause {
    fn id(&self) -> u8 {
        24
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(le_i8, |is_paused| SvcSetPause { is_paused })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i8(self.is_paused);

        writer.data
    }
}
