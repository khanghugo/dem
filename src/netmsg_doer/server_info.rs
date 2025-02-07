use super::*;

impl Doer for SvcServerInfo {
    fn id(&self) -> u8 {
        11
    }

    fn parse(i: &[u8], aux: AuxRefCell) -> Result<Self> {
        map(
            tuple((
                le_i32,
                le_i32,
                le_i32,
                take(16usize),
                le_u8,
                le_u8,
                le_u8,
                null_string,
                null_string,
                null_string,
                null_string,
                le_u8,
            )),
            |(
                protocol,
                spawn_count,
                map_checksum,
                client_dll_hash,
                max_players,
                player_index,
                is_deathmatch,
                game_dir,
                hostname,
                map_file_name,
                map_cycle,
                unknown,
            )| {
                let mut aux = aux.borrow_mut();

                // mutate max_client
                aux.max_client = max_players;

                Self {
                    protocol,
                    spawn_count,
                    map_checksum,
                    client_dll_hash: client_dll_hash.into(),
                    max_players,
                    player_index,
                    is_deathmatch,
                    game_dir: game_dir.to_vec(),
                    hostname: hostname.to_vec(),
                    map_file_name: map_file_name.to_vec(),
                    map_cycle: map_cycle.to_vec(),
                    unknown,
                }
            },
        )(i)
    }

    fn write(&self, _: AuxRefCell) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_i32(self.protocol);
        writer.append_i32(self.spawn_count);
        writer.append_i32(self.map_checksum);
        writer.append_u8_slice(self.client_dll_hash.padded(16).as_slice());
        writer.append_u8(self.max_players);
        writer.append_u8(self.player_index);
        writer.append_u8(self.is_deathmatch);
        writer.append_u8_slice(&self.game_dir);
        writer.append_u8_slice(&self.hostname);
        writer.append_u8_slice(&self.map_file_name);
        writer.append_u8_slice(&self.map_cycle);
        writer.append_u8(self.unknown);

        writer.data
    }
}
