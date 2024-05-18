use super::*;

impl Doer for SvcEventReliable {
    fn id(&self) -> u8 {
        21
    }

    fn parse<'a>(i: &'a [u8], aux: &'a RefCell<Aux>) -> Result<'a, Self> {
        let aux = aux.borrow();

        let mut br = BitReader::new(i);

        let event_index = br.read_n_bit(10).to_owned();
        let event_args = parse_delta(aux.delta_decoders.get("event_t\0").unwrap(), &mut br);
        let has_fire_time = br.read_1_bit();
        let fire_time = if has_fire_time {
            Some(br.read_n_bit(16).to_owned())
        } else {
            None
        };

        let range = br.get_consumed_bytes();
        let (i, _) = take(range)(i)?;

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

    fn write(&self, aux: &RefCell<Aux>) -> ByteVec {
        let aux = aux.borrow();

        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(self.id());

        bw.append_vec(&self.event_index);
        write_delta(
            &self.event_args,
            aux.delta_decoders.get("event_t\0").unwrap(),
            &mut bw,
        );

        bw.append_bit(self.has_fire_time);
        if self.has_fire_time {
            bw.append_vec(self.fire_time.as_ref().unwrap());
        }

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
