use crate::types::EventS;

use super::*;

impl Doer for SvcEvent {
    fn id(&self) -> u8 {
        3
    }

    fn parse(i: &[u8], aux: Aux) -> Result<Self> {
        let mut br = BitReader::new(i);

        let event_count = br.read_n_bit(5).to_owned();

        let events = (0..event_count.to_u8())
            .map(|_| {
                let event_index = br.read_n_bit(10).to_owned();
                let has_packet_index = br.read_1_bit();
                let packet_index = if has_packet_index {
                    Some(br.read_n_bit(11).to_owned())
                } else {
                    None
                };
                let has_delta = if has_packet_index {
                    Some(br.read_1_bit())
                } else {
                    None
                };
                let delta = if has_delta.is_some() && has_delta.unwrap() {
                    Some(parse_delta(
                        aux.delta_decoders.get("event_t\0").unwrap(),
                        &mut br,
                    ))
                } else {
                    None
                };
                let has_fire_time = br.read_1_bit();
                let fire_time = if has_fire_time {
                    Some(br.read_n_bit(16).to_owned())
                } else {
                    None
                };

                EventS {
                    event_index,
                    has_packet_index,
                    packet_index,
                    has_delta,
                    delta,
                    has_fire_time,
                    fire_time,
                }
            })
            .collect();

        let range = br.get_consumed_bytes();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            Self {
                event_count,
                events,
            },
        ))
    }

    fn write(&self, aux: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(self.id());

        bw.append_vec(&self.event_count);

        for event in &self.events {
            bw.append_vec(&event.event_index);
            bw.append_bit(event.has_packet_index);

            if event.has_packet_index {
                bw.append_vec(event.packet_index.as_ref().unwrap());
                bw.append_bit(event.has_delta.unwrap());

                if event.has_delta.unwrap() {
                    write_delta(
                        event.delta.as_ref().unwrap(),
                        aux.delta_decoders.get("event_t\0").unwrap(),
                        &mut bw,
                    );
                }
            }

            bw.append_bit(event.has_fire_time);
            if event.has_fire_time {
                bw.append_vec(event.fire_time.as_ref().unwrap());
            }
        }

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
