use crate::types::EntityStateDelta;

use super::*;

impl Doer for SvcDeltaPacketEntities {
    fn id(&self) -> u8 {
        41
    }

    fn parse(i: &[u8], aux: Aux) -> Result<Self> {
        let mut br = BitReader::new(i);

        let entity_count = br.read_n_bit(16).to_owned();
        let delta_sequence = br.read_n_bit(8).to_owned();

        let mut entity_index: u16 = 0;
        let mut entity_states: Vec<EntityStateDelta> = vec![];

        loop {
            let footer = br.peek_n_bits(16).to_u16();
            if footer == 0 {
                br.read_n_bit(16);
                break;
            }

            let remove_entity = br.read_1_bit();
            let is_absolute_entity_index = br.read_1_bit();

            let (absolute_entity_index, entity_index_difference) = if is_absolute_entity_index {
                let idx = br.read_n_bit(11).to_owned();
                entity_index = idx.to_u16();
                (Some(idx), None)
            } else {
                let diff = br.read_n_bit(6).to_owned();
                entity_index += diff.to_u16();
                (None, Some(diff))
            };

            if remove_entity {
                entity_states.push(EntityStateDelta {
                    entity_index,
                    remove_entity,
                    is_absolute_entity_index,
                    absolute_entity_index,
                    entity_index_difference,
                    has_custom_delta: None,
                    delta: None,
                });
                continue;
            }

            let has_custom_delta = br.read_1_bit();
            let between = entity_index > 0 && entity_index <= aux.max_client as u16;

            let delta = if between {
                parse_delta(
                    aux.delta_decoders.get("entity_state_player_t\0").unwrap(),
                    &mut br,
                )
            } else if has_custom_delta {
                parse_delta(
                    aux.delta_decoders.get("custom_entity_state_t\0").unwrap(),
                    &mut br,
                )
            } else {
                parse_delta(aux.delta_decoders.get("entity_state_t\0").unwrap(), &mut br)
            };

            entity_states.push(EntityStateDelta {
                entity_index,
                remove_entity,
                is_absolute_entity_index,
                absolute_entity_index,
                entity_index_difference,
                has_custom_delta: Some(has_custom_delta),
                delta: Some(delta),
            });
        }

        let range = br.get_consumed_bytes();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            Self {
                entity_count,
                delta_sequence,
                entity_states,
            },
        ))
    }

    fn write(&self, aux: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(self.id());

        bw.append_vec(&self.entity_count);
        bw.append_vec(&self.delta_sequence);

        for entity in &self.entity_states {
            bw.append_bit(entity.remove_entity);
            bw.append_bit(entity.is_absolute_entity_index);

            if entity.is_absolute_entity_index {
                bw.append_vec(entity.absolute_entity_index.as_ref().unwrap());
            } else {
                bw.append_vec(entity.entity_index_difference.as_ref().unwrap());
            }

            if entity.remove_entity {
                continue;
            }

            bw.append_bit(entity.has_custom_delta.unwrap());

            let between = entity.entity_index > 0 && entity.entity_index <= aux.max_client as u16;
            if between {
                write_delta(
                    entity.delta.as_ref().unwrap(),
                    aux.delta_decoders.get("entity_state_player_t\0").unwrap(),
                    &mut bw,
                )
            } else if entity.has_custom_delta.unwrap() {
                write_delta(
                    entity.delta.as_ref().unwrap(),
                    aux.delta_decoders.get("custom_entity_state_t\0").unwrap(),
                    &mut bw,
                )
            } else {
                write_delta(
                    entity.delta.as_ref().unwrap(),
                    aux.delta_decoders.get("entity_state_t\0").unwrap(),
                    &mut bw,
                )
            }
        }

        // Remember to append 16 bits of 0
        use bitvec::bitvec;
        use bitvec::prelude::Lsb0;
        bw.append_vec(&bitvec![u8, Lsb0; 0; 16]);

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
