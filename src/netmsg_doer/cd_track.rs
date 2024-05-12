use super::*;

impl Doer<SvcCdTrack> for SvcCdTrack {
    fn id(&self) -> u8 {
        32
    }

    fn parse(i: &[u8], _: Aux) -> Result<SvcCdTrack> {
        map(tuple((le_i8, le_i8)), |(track, loop_track)| SvcCdTrack {
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
