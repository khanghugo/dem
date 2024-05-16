use super::*;

impl Doer for SvcNewUserMsg {
    fn id(&self) -> u8 {
        39
    }

    fn parse(i: &[u8], mut aux: Aux) -> Result<Self> {
        map(
            tuple((le_u8, le_i8, take(16usize))),
            |(index, size, name): (_, _, &[u8])| {
                let msg = Self {
                    index,
                    size,
                    name: name.to_vec(),
                };

                // mutate custom_messages
                aux.custom_messages.remove(&index);
                aux.custom_messages.insert(index, msg.to_owned());

                msg
            },
        )(i)
    }

    fn write(&self, _: Aux) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        writer.append_u8(self.index);
        writer.append_i8(self.size);
        writer.append_u8_slice(&self.name);

        writer.data
    }
}
