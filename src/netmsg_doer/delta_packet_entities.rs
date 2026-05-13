use crate::types::EntityStateDelta;

use super::*;

impl Doer for SvcDeltaPacketEntities {
    fn id(&self) -> u8 {
        41
    }

    fn parse<'a>(i: &'a [u8], aux: &mut DemoGlobalState) -> NomResult<'a, Self> {
        let mut br = BitReader::new(i);

        let entity_count = br.read_n_bit(16).to_u16();
        let delta_sequence = br.read_n_bit(8).to_u8();

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
                let idx = br.read_n_bit(11).to_u16();
                entity_index = idx;
                (Some(idx), None)
            } else {
                let diff = br.read_n_bit(6).to_u8();
                entity_index += diff as u16;
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

    fn write(&self, aux: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(self.id());

        bw.append_u16(self.entity_count);
        bw.append_u8(self.delta_sequence);

        for entity in &self.entity_states {
            bw.append_bit(entity.remove_entity);
            bw.append_bit(entity.is_absolute_entity_index);

            if entity.is_absolute_entity_index {
                bw.append_u11(entity.absolute_entity_index.unwrap());
            } else {
                bw.append_u6(entity.entity_index_difference.unwrap());
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
        bw.append_u16(0);

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
