use crate::types::{Consistency, Resource, SvcResourceList};

use super::*;

impl Doer for SvcResourceList {
    fn id(&self) -> u8 {
        43
    }

    fn parse<'a>(i: &'a [u8], _: &mut DemoGlobalState) -> NomResult<'a, Self> {
        let mut br = BitReader::new(i);

        let resource_count = br.read_n_bit(12).to_u16();

        let resources: Vec<Resource> = (0..resource_count)
            .map(|_| {
                let type_ = br.read_n_bit(4).to_u8();
                let name = br.read_string().get_string();
                let index = br.read_n_bit(12).to_u16();
                let size = br.read_n_bit(24).to_u32();
                let flags = br.read_n_bit(3).to_u8();
                let md5_hash = if flags & 4 != 0 {
                    Some(br.read_bytes::<16>())
                } else {
                    None
                };
                let has_extra_info = br.read_1_bit();
                let extra_info = if has_extra_info {
                    Some(br.read_bytes::<32>())
                } else {
                    None
                };

                Resource {
                    type_,
                    name,
                    index,
                    size,
                    flags,
                    md5_hash,
                    has_extra_info,
                    extra_info,
                }
            })
            .collect();

        let mut consistencies: Vec<Consistency> = vec![];

        if br.read_1_bit() {
            while br.read_1_bit() {
                let is_short_index = br.read_1_bit();

                let (short_index, long_index) = if is_short_index {
                    (Some(br.read_n_bit(5).to_u8()), None)
                } else {
                    (None, Some(br.read_n_bit(10).to_u16()))
                };

                consistencies.push(Consistency {
                    is_short_index: Some(is_short_index),
                    short_index,
                    long_index,
                });
            }
        }

        let (i, _) = take(br.get_consumed_bytes())(i)?;

        Ok((
            i,
            Self {
                resource_count,
                resources,
                consistencies,
            },
        ))
    }

    fn write(&self, _: &DemoGlobalState) -> ByteVec {
        let mut writer = ByteWriter::new();

        writer.append_u8(self.id());

        let mut bw = BitWriter::new();
        bw.append_u12(self.resource_count);

        for resource in &self.resources {
            bw.append_u4(resource.type_);
            bw.append_string(&resource.name);
            bw.append_u12(resource.index);
            bw.append_u24(resource.size);

            let should_add_md5_hash = resource.flags & 4 != 0;

            bw.append_u3(resource.flags);

            if should_add_md5_hash {
                bw.append_bytes(resource.md5_hash.unwrap());
            }

            bw.append_bit(resource.has_extra_info);

            if resource.has_extra_info {
                bw.append_bytes(resource.extra_info.unwrap());
            }
        }

        // First read bit.
        if self.consistencies.is_empty() {
            bw.append_bit(false);
        } else {
            bw.append_bit(true);
        }

        for consistency in &self.consistencies {
            // for eery consistency, add true bit first
            bw.append_bit(true);

            bw.append_bit(consistency.is_short_index.unwrap());
            if consistency.is_short_index.unwrap() {
                bw.append_u5(consistency.short_index.unwrap());
            } else {
                bw.append_u10(consistency.long_index.unwrap());
            }
        }

        // Last bit for consistency.
        bw.append_bit(false);

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
