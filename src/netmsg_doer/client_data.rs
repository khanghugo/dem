use crate::types::ClientDataWeaponData;

use super::*;

impl Doer for SvcClientData {
    fn id(&self) -> u8 {
        15
    }

    fn parse(i: &[u8], aux: AuxRefCell) -> Result<Self> {
        let mut br = BitReader::new(i);
        let aux = aux.borrow();

        let has_delta_update_mask = br.read_1_bit();
        let delta_update_mask = if has_delta_update_mask {
            Some(br.read_n_bit(8).to_owned())
        } else {
            None
        };

        let client_data = parse_delta(aux.delta_decoders.get("clientdata_t\0").unwrap(), &mut br);

        // This is a vector unlike THE docs.
        let mut weapon_data: Vec<ClientDataWeaponData> = vec![];
        while br.read_1_bit() {
            let weapon_index = br.read_n_bit(6).to_owned();
            let delta = parse_delta(aux.delta_decoders.get("weapon_data_t\0").unwrap(), &mut br);

            weapon_data.push(ClientDataWeaponData {
                weapon_index,
                weapon_data: delta,
            });
        }

        let weapon_data = if weapon_data.is_empty() {
            None
        } else {
            Some(weapon_data)
        };

        // Remember to write the last "false" bit.

        let range = br.get_consumed_bytes();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            Self {
                has_delta_update_mask,
                delta_update_mask,
                client_data,
                weapon_data,
            },
        ))
    }

    fn write(&self, aux: AuxRefCell) -> ByteVec {
        let aux = aux.borrow();

        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(self.id());

        bw.append_bit(self.has_delta_update_mask);

        if self.has_delta_update_mask {
            bw.append_vec(self.delta_update_mask.as_ref().unwrap());
        }

        write_delta(
            &self.client_data,
            aux.delta_decoders.get("clientdata_t\0").unwrap(),
            &mut bw,
        );

        if let Some(weapon_data) = &self.weapon_data {
            for data in weapon_data {
                bw.append_bit(true);
                bw.append_vec(&data.weapon_index);
                write_delta(
                    &data.weapon_data,
                    aux.delta_decoders.get("weapon_data_t\0").unwrap(),
                    &mut bw,
                );
            }
        }

        // false bit for weapon data
        bw.append_bit(false);

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
