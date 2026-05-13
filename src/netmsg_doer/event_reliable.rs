use super::*;

impl Doer for SvcEventReliable {
    fn id(&self) -> u8 {
        21
    }

    fn parse<'a>(i: &'a [u8], aux: &mut DemoGlobalState) -> NomResult<'a, Self> {
        let mut br = BitReader::new(i);

        let event_index = br.read_n_bit(10).to_u16();
        let event_args = parse_delta(aux.delta_decoders.get("event_t\0").unwrap(), &mut br);
        let has_fire_time = br.read_1_bit();
        let fire_time = if has_fire_time {
            Some(br.read_n_bit(16).to_u16())
        } else {
            None
        };

        let range = br.get_consumed_bytes();
        let (i, _) = take(range).parse(i)?;

        Ok((
            i,
            Self {
                event_index,
                event_args,
                has_fire_time,
                fire_time,
            },
        ))
    }

    fn write(&self, aux: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(self.id());

        bw.append_u10(self.event_index);

        write_delta(
            &self.event_args,
            aux.delta_decoders.get("event_t\0").unwrap(),
            &mut bw,
        );

        bw.append_bit(self.has_fire_time);
        if self.has_fire_time {
            bw.append_u16(self.fire_time.unwrap());
        }

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
