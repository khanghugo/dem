use crate::types::{EntityState, SvcPacketEntities};

use super::*;

impl Doer for SvcPacketEntities {
    fn id(&self) -> u8 {
        40
    }

    fn parse<'a>(i: &'a [u8], aux: &mut DemoGlobalState) -> NomResult<'a, Self> {
        let mut br = BitReader::new(i);

        let entity_count = br.read_n_bit(16).to_u16();
        let mut entity_index = 0;
        let mut entity_states: Vec<EntityState> = vec![];

        loop {
            let footer = br.peek_n_bits(16).to_u16();
            if footer == 0 {
                br.read_n_bit(16);
                break;
            }

            let increment_entity_number = br.read_1_bit();
            let is_absolute_entity_index = if increment_entity_number {
                entity_index += 1;
                None
            } else {
                Some(br.read_1_bit())
            };
            let (absolute_entity_index, entity_index_difference) =
                if is_absolute_entity_index.is_some() {
                    if !is_absolute_entity_index.unwrap() {
                        let val = br.read_n_bit(6).to_u8();
                        entity_index += val as u16;
                        (None, Some(val))
                    } else {
                        let val = br.read_n_bit(11).to_u16();
                        entity_index = val;
                        (Some(val), None)
                    }
                } else {
                    (None, None)
                };

            let has_custom_delta = br.read_1_bit();
            let has_baseline_index = br.read_1_bit();
            let baseline_index = if has_baseline_index {
                Some(br.read_n_bit(6).to_u8())
            } else {
                None
            };
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

            entity_states.push(EntityState {
                entity_index,
                increment_entity_number,
                is_absolute_entity_index,
                absolute_entity_index,
                entity_index_difference,
                has_custom_delta,
                has_baseline_index,
                baseline_index,
                delta,
            })
        }

        let range = br.get_consumed_bytes();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            Self {
                entity_count,
                entity_states,
            },
        ))
    }

    fn write(&self, aux: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(self.id());

        bw.append_u16(self.entity_count);

        for entity in &self.entity_states {
            bw.append_bit(entity.increment_entity_number);

            if !entity.increment_entity_number {
                bw.append_bit(entity.is_absolute_entity_index.unwrap());

                if entity.is_absolute_entity_index.unwrap() {
                    bw.append_u11(entity.absolute_entity_index.unwrap());
                } else {
                    bw.append_u6(entity.entity_index_difference.unwrap());
                }
            }

            bw.append_bit(entity.has_custom_delta);
            bw.append_bit(entity.has_baseline_index);

            if entity.has_baseline_index {
                bw.append_u6(entity.baseline_index.unwrap());
            }

            let between = entity.entity_index > 0 && entity.entity_index <= aux.max_client as u16;
            if between {
                write_delta(
                    &entity.delta,
                    aux.delta_decoders.get("entity_state_player_t\0").unwrap(),
                    &mut bw,
                )
            } else if entity.has_custom_delta {
                write_delta(
                    &entity.delta,
                    aux.delta_decoders.get("custom_entity_state_t\0").unwrap(),
                    &mut bw,
                )
            } else {
                write_delta(
                    &entity.delta,
                    aux.delta_decoders.get("entity_state_t\0").unwrap(),
                    &mut bw,
                )
            }
        }

        // end
        bw.append_u16(0);

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
