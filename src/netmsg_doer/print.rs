use super::*;

impl Doer for SvcPrint {
    fn id(&self) -> u8 {
        8
    }

    fn parse(i: &[u8], _: AuxRefCell) -> Result<Self> {
        map(null_string, |message| SvcPrint {
            message: message.to_vec(),
        })(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.message);

        writer.data
    }
}
