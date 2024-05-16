use std::str::from_utf8;

use super::*;

impl Doer for SvcDeltaDescription {
    fn id(&self) -> u8 {
        14
    }

    fn parse(i: &[u8], mut aux: Aux) -> Result<Self> {
        let (i, name) = null_string(i)?;
        let (i, total_fields) = le_u16(i)?;

        let clone = i;

        // Delta description is usually in LOADING section and first frame message.
        // It will detail the deltas being used and its index for correct decoding.
        // So this would be the only message that modifies the delta decode table.

        let mut br = BitReader::new(i);
        let data: Vec<Delta> = (0..total_fields)
            .map(|_| {
                parse_delta(
                    aux.delta_decoders.get("delta_description_t\0").unwrap(),
                    &mut br,
                )
            })
            .collect();

        let decoder: DeltaDecoder = data
            .iter()
            .map(|entry| {
                DeltaDecoderS {
                    name: entry.get("name").unwrap().to_owned(),
                    bits: u32::from_le_bytes(
                        entry.get("bits").unwrap().as_slice().try_into().unwrap(),
                    ), // heh
                    divisor: f32::from_le_bytes(
                        entry.get("divisor").unwrap().as_slice().try_into().unwrap(),
                    ),
                    flags: u32::from_le_bytes(
                        entry.get("flags").unwrap().as_slice().try_into().unwrap(),
                    ),
                }
            })
            .collect();

        let range = br.get_consumed_bytes();
        let clone = &clone[..range];
        let (i, _) = take(range)(i)?;

        // mutate delta_decoders
        aux.delta_decoders
            .insert(from_utf8(name).unwrap().to_owned(), decoder.clone());

        Ok((
            i,
            Self {
                name: name.to_vec(),
                total_fields,
                fields: decoder,
                clone: clone.to_vec(),
            },
        ))
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8_slice(&self.name);
        writer.append_u16(self.total_fields);

        // This is intentionally done like this because I don't think anyone
        // would try to modify delta description.
        writer.append_u8_slice(&self.clone);

        writer.data
    }
}
