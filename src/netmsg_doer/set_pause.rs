use super::*;

impl Doer for SvcSetPause {
    fn id(&self) -> u8 {
        24
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        map(le_i8, |is_paused| SvcSetPause { is_paused })(i)
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i8(self.is_paused);

        writer.data
    }
}
