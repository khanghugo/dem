use super::*;

impl Doer for SvcRoomType {
    fn id(&self) -> u8 {
        37
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(le_u16, |room_type| SvcRoomType { room_type })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u16(self.room_type);

        writer.data
    }
}
