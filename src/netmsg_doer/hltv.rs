use super::*;

impl Doer for SvcHltv {
    fn id(&self) -> u8 {
        50
    }

    fn parse(i: &[u8], aux: AuxRefCell) -> Result<Self> {
        let mut aux = aux.borrow_mut();

        aux.is_hltv = true;

        map(le_u8, |mode| SvcHltv { mode })(i)
    }

    fn write(&self, aux: AuxRefCell) -> ByteVec {
        let mut aux = aux.borrow_mut();

        aux.is_hltv = true;

        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.mode);

        writer.data
    }
}
