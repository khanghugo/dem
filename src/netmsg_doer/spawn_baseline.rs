use crate::types::{Delta, EntityS};

use super::*;

impl Doer for SvcSpawnBaseline {
    fn id(&self) -> u8 {
        22
    }

    fn parse<'a>(i: &'a [u8], aux: &mut DemoGlobalState) -> NomResult<'a, Self> {
        let mut br = BitReader::new(i);
        let mut entities: Vec<EntityS> = vec![];

        while br.peek_n_bits(16).to_u32() != (1 << 16) - 1 {
            let index = br.read_n_bit(11).to_u16();
            let entity_index = index;

            let between = index > 0 && index <= aux.max_client as u16;
            let type_ = br.read_n_bit(2).to_u8();

            let delta = if type_ & 1 != 0 {
                if between {
                    parse_delta(
                        aux.delta_decoders.get("entity_state_player_t\0").unwrap(),
                        &mut br,
                    )
                } else {
                    parse_delta(aux.delta_decoders.get("entity_state_t\0").unwrap(), &mut br)
                }
            } else {
                parse_delta(
                    aux.delta_decoders.get("custom_entity_state_t\0").unwrap(),
                    &mut br,
                )
            };

            let res = EntityS {
                index,
                entity_index,
                type_,
                delta,
            };

            entities.push(res);
        }

        // Footer | last entity = (1 << 16) - 1
        br.read_n_bit(16);

        let total_extra_data = br.read_n_bit(6).to_u8();

        let extra_data_description = aux.delta_decoders.get("entity_state_t\0").unwrap();
        let extra_data: Vec<Delta> = (0..total_extra_data)
            .map(|_| parse_delta(extra_data_description, &mut br))
            .collect();

        let range = br.get_consumed_bytes();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            Self {
                entities,
                total_extra_data,
                extra_data,
            },
        ))
    }

    fn write(&self, aux: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());
        let mut bw = BitWriter::new();

        for entity in &self.entities {
            let between = entity.index > 0 && entity.index <= aux.max_client as u16;

            bw.append_u11(entity.index);
            bw.append_u2(entity.type_); // heh

            if entity.type_ & 1 != 0 {
                if between {
                    write_delta(
                        &entity.delta,
                        aux.delta_decoders.get("entity_state_player_t\0").unwrap(),
                        &mut bw,
                    )
                } else {
                    write_delta(
                        &entity.delta,
                        aux.delta_decoders.get("entity_state_t\0").unwrap(),
                        &mut bw,
                    )
                }
            } else {
                write_delta(
                    &entity.delta,
                    aux.delta_decoders.get("custom_entity_state_t\0").unwrap(),
                    &mut bw,
                )
            }
        }

        // end
        // append all 0xFFFF
        bw.append_u16(0xFFFF);

        bw.append_u6(self.total_extra_data);

        let extra_data_description = aux.delta_decoders.get("entity_state_t\0").unwrap();
        for data in &self.extra_data {
            write_delta(data, extra_data_description, &mut bw)
        }

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
