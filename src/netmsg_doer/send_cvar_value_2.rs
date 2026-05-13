use super::*;

impl Doer for SvcSendCvarValue2 {
    fn id(&self) -> u8 {
        58
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map((le_u32, null_string), |(request_id, name)| {
            SvcSendCvarValue2 {
                request_id,
                name: name.into(),
            }
        })
        .parse(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u32(self.request_id);
        writer.append_u8_slice(self.name.as_slice());

        writer.data
    }
}
