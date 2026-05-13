use super::*;

impl Doer for SvcRoomType {
    fn id(&self) -> u8 {
        37
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(le_u16, |room_type| SvcRoomType { room_type }).parse(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u16(self.room_type);

        writer.data
    }
}
