use crate::types::PingS;

use super::*;

impl Doer for SvcPings {
    fn id(&self) -> u8 {
        17
    }

    fn parse(i: &[u8], _: Aux) -> Result<Self> {
        let mut br = BitReader::new(i);
        let mut pings: Vec<PingS> = vec![];

        while br.read_1_bit() {
            pings.push(PingS {
                has_ping_data: true,
                player_id: Some(br.read_n_bit(8).to_u8()),
                ping: Some(br.read_n_bit(8).to_u8()),
                loss: Some(br.read_n_bit(8).to_u8()),
            })
        }

        // If we exit the loop, it means we already read the has_ping_data = false bit.

        // Last element.
        pings.push(PingS {
            has_ping_data: false,
            player_id: None,
            ping: None,
            loss: None,
        });

        // Don't forget
        let (i, _) = take(br.get_consumed_bytes())(i)?;

        Ok((i, SvcPings { pings }))
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        let mut bw = BitWriter::new();

        for ping in &self.pings {
            if ping.has_ping_data {
                bw.append_bit(true);
                bw.append_u8(ping.player_id.unwrap());
                bw.append_u8(ping.ping.unwrap());
                bw.append_u8(ping.loss.unwrap());
            } else {
                bw.append_bit(false);
            }
        }

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
