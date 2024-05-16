use super::*;

impl Doer for SvcCdTrack {
    fn id(&self) -> u8 {
        32
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        map(tuple((le_i8, le_i8)), |(track, loop_track)| Self {
            track,
            loop_track,
        })(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i8(self.track);
        writer.append_i8(self.loop_track);

        writer.data
    }
}
