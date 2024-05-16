use crate::types::EntityS;

use super::*;

impl Doer for SvcSpawnBaseline {
    fn id(&self) -> u8 {
        22
    }

    fn parse(i: &[u8], aux: Aux) -> Result<Self> {
        let mut br = BitReader::new(i);
        let mut entities: Vec<EntityS> = vec![];

        while br.peek_n_bits(16).to_u32() != (1 << 16) - 1 {
            let index = br.read_n_bit(11).to_owned();
            let entity_index = index.to_u16();

            let between = index.to_u16() > 0 && index.to_u16() <= aux.max_client as u16;
            let type_ = br.read_n_bit(2).to_owned();

            let delta = if type_.to_u8() & 1 != 0 {
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
                index: index.clone(),
                entity_index,
                type_,
                delta,
            };

            entities.push(res);
        }

        // Footer | last entity = (1 << 16) - 1
        br.read_n_bit(16);

        let total_extra_data = br.read_n_bit(6).to_owned();

        let extra_data_description = aux.delta_decoders.get("entity_state_t\0").unwrap();
        let extra_data: Vec<Delta> = (0..total_extra_data.to_u8())
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

    fn write(&self, aux: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());
        let mut bw = BitWriter::new();

        for entity in &self.entities {
            let between =
                entity.index.to_u16() > 0 && entity.index.to_u16() <= aux.max_client as u16;

            bw.append_vec(&entity.index);
            bw.append_slice(&entity.type_); // heh

            if entity.type_.to_u8() & 1 != 0 {
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

        use bitvec::bitvec;
        use bitvec::prelude::Lsb0;
        bw.append_vec(&bitvec![u8, Lsb0; 1; 16]);

        bw.append_vec(&self.total_extra_data);

        let extra_data_description = aux.delta_decoders.get("entity_state_t\0").unwrap();
        for data in &self.extra_data {
            write_delta(data, extra_data_description, &mut bw)
        }

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
