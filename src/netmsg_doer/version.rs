use super::*;

impl Doer for SvcVersion {
    fn id(&self) -> u8 {
        4
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(le_u32, |protocol_version| SvcVersion { protocol_version })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u32(self.protocol_version);

        writer.data
    }
}
