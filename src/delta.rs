use std::str::from_utf8;

use crate::{
    bit::{BitReader, BitSlice, BitSliceCast, BitWriter},
    types::{Delta, DeltaDecoder, DeltaDecoderS, DeltaType},
};

pub fn parse_delta(dd: &DeltaDecoder, br: &mut BitReader) -> Delta {
    let mut res: Delta = Delta::new();

    let mask_byte_count = br.read_n_bit(3).to_u8() as usize;
    let mask_bytes: Vec<u8> = (0..mask_byte_count)
        .map(|_| br.read_n_bit(8).to_u8())
        .collect();

    // for i in 0..mask_byte_count {
    for (i, mask_byte) in mask_bytes.iter().enumerate().take(mask_byte_count) {
        for j in 0..8 {
            let index = j + i * 8;

            if index == dd.len() {
                return res;
            }

            if (mask_byte & (1 << j)) != 0 {
                let description = &dd[index];
                let key = from_utf8(&description.name).unwrap().to_owned();
                let value = parse_delta_field(description, &mut res, br);
                res.insert(key, value);
            }
        }
    }

    res
}

macro_rules! flag {
    ($lhs:expr, $rhs:expr) => {{
        $lhs as u32 & $rhs as u32 != 0
    }};
}

fn parse_delta_field(description: &DeltaDecoderS, res: &mut Delta, br: &mut BitReader) -> Vec<u8> {
    let lhs = description.flags;

    let is_signed = flag!(lhs, DeltaType::Signed);
    let is_byte = flag!(lhs, DeltaType::Byte);
    let is_short = flag!(lhs, DeltaType::Short);
    let is_integer = flag!(lhs, DeltaType::Integer);
    let is_some_float = flag!(lhs, DeltaType::Float)
        || flag!(lhs, DeltaType::TimeWindow8)
        || flag!(lhs, DeltaType::TimeWindowBig);
    let is_angle = flag!(lhs, DeltaType::Angle);
    let is_string = flag!(lhs, DeltaType::String);

    if is_byte {
        if is_signed {
            let sign = if br.read_1_bit() { -1 } else { 1 };
            let value = br.read_n_bit(description.bits as usize - 1).to_u8();
            let res_value = ((sign * value as i8) / description.divisor as i8).to_le_bytes();
            res_value.to_vec()
        } else {
            let value = (br.read_n_bit(description.bits as usize)).to_u8();
            let res_value = (value / description.divisor as u8).to_le_bytes();
            res_value.to_vec()
        }
    } else if is_short {
        if is_signed {
            let sign = if br.read_1_bit() { -1 } else { 1 };
            let value = (br.read_n_bit(description.bits as usize - 1)).to_u16();
            let res_value = ((sign * value as i16) / description.divisor as i16).to_le_bytes();
            res_value.to_vec()
        } else {
            let value = (br.read_n_bit(description.bits as usize)).to_u16();
            let res_value = (value / description.divisor as u16).to_le_bytes();
            res_value.to_vec()
        }
    } else if is_integer {
        if is_signed {
            let sign = if br.read_1_bit() { -1 } else { 1 };
            let value = (br.read_n_bit(description.bits as usize - 1)).to_u32();
            let res_value = ((sign * value as i32) / description.divisor as i32).to_le_bytes();
            res_value.to_vec()
        } else {
            let value = (br.read_n_bit(description.bits as usize)).to_u32();
            let res_value = (value / description.divisor as u32).to_le_bytes();
            res_value.to_vec()
        }
    } else if is_some_float {
        if is_signed {
            let sign = if br.read_1_bit() { -1 } else { 1 };
            let value = (br.read_n_bit(description.bits as usize - 1)).to_u32();
            let res_value = (((sign * value as i32) as f32) / (description.divisor)).to_le_bytes();
            res_value.to_vec()
        } else {
            let value = (br.read_n_bit(description.bits as usize)).to_u32();
            let res_value = ((value as f32) / (description.divisor)).to_le_bytes();
            res_value.to_vec()
        }
    } else if is_angle {
        let value = (br.read_n_bit(description.bits as usize)).to_u32();
        let multiplier = 360f32 / ((1 << description.bits) as f32);
        let res_value = (value as f32 * multiplier).to_le_bytes();
        res_value.to_vec()
    } else if is_string {
        bitslice_to_u8_vec(br.read_string())
    } else {
        unreachable!("Encoded value does not match any types. Should this happens?");
    }
}

fn bitslice_to_u8_vec(i: &BitSlice) -> Vec<u8> {
    i.chunks(8).map(|chunk| chunk.to_u8()).collect()
}

pub fn write_delta(delta: &Delta, delta_decoder: &DeltaDecoder, bw: &mut BitWriter) {
    // Consider this like a modulo.
    // Delta with description of index 13 is byte_mask[13 / 8] at 13 % 8.
    // Byte mask count adds accordingly if we have entry with biggest index number.
    let mut byte_mask = [0u8; 8];
    let mut byte_mask_count = 0u8;
    let mut yes_data = false;

    // This step marks which delta field will be encoded.
    for key in delta.keys() {
        let (index, _) = find_decoder(key.as_bytes(), delta_decoder).unwrap();
        let quotient = index / 8;
        let remainder = index % 8;

        byte_mask[quotient] |= 1 << remainder;
        byte_mask_count = byte_mask_count.max(quotient as u8);
        yes_data = true;
    }

    // Because we start counting at 0, we need to offset this by 1 for correct length.
    if yes_data {
        byte_mask_count += 1;
    }

    bw.append_u32_range(byte_mask_count as u32, 3);
    for i in 0..byte_mask_count {
        bw.append_u8(byte_mask[i as usize]);
    }

    // We have to write delta by the described order.
    for description in delta_decoder {
        if delta.contains_key(from_utf8(&description.name).unwrap()) {
            write_delta_field(description, find_delta_value(&description.name, delta), bw);
        }
    }
}

fn write_delta_field(description: &DeltaDecoderS, value: &[u8], bw: &mut BitWriter) {
    let lhs = description.flags;

    let is_signed = flag!(lhs, DeltaType::Signed);
    let is_byte = flag!(lhs, DeltaType::Byte);
    let is_short = flag!(lhs, DeltaType::Short);
    let is_integer = flag!(lhs, DeltaType::Integer);
    let is_angle = flag!(lhs, DeltaType::Angle);
    let is_some_float = flag!(lhs, DeltaType::Float)
        || flag!(lhs, DeltaType::TimeWindow8)
        || flag!(lhs, DeltaType::TimeWindowBig);
    let is_string = flag!(lhs, DeltaType::String);

    if is_byte {
        let bytes: [u8; 1] = value[..1].try_into().unwrap();
        if is_signed {
            let res_value = i8::from_le_bytes(bytes);
            let signed_value = res_value * description.divisor as i8;
            let is_negative = signed_value < 0;

            let value = if is_negative {
                bw.append_bit(true);
                -signed_value
            } else {
                bw.append_bit(false);
                signed_value
            };

            // value is positive so cast unsigned without side effects.
            bw.append_u32_range(value as u32, description.bits - 1);
        } else {
            let res_value = u8::from_le_bytes(bytes);
            let value = res_value * description.divisor as u8;

            bw.append_u32_range(value as u32, description.bits);
        }
    } else if is_short {
        let bytes: [u8; 2] = value[..2].try_into().unwrap();
        if is_signed {
            let res_value = i16::from_le_bytes(bytes);
            let signed_value = res_value * description.divisor as i16;
            let is_negative = signed_value < 0;

            let value = if is_negative {
                bw.append_bit(true);
                -signed_value
            } else {
                bw.append_bit(false);
                signed_value
            };

            bw.append_u32_range(value as u32, description.bits - 1);
        } else {
            let res_value = u16::from_le_bytes(bytes);
            let value = res_value * description.divisor as u16;

            bw.append_u32_range(value as u32, description.bits);
        }
    } else if is_integer {
        let bytes: [u8; 4] = value[..4].try_into().unwrap();
        if is_signed {
            let res_value = i32::from_le_bytes(bytes);
            let signed_value = res_value * description.divisor as i32;
            let is_negative = signed_value < 0;

            let value = if is_negative {
                bw.append_bit(true);
                -signed_value
            } else {
                bw.append_bit(false);
                signed_value
            };

            bw.append_u32_range(value as u32, description.bits - 1);
        } else {
            let res_value = u32::from_le_bytes(bytes);
            let value = res_value * description.divisor as u32;

            bw.append_u32_range(value, description.bits);
        }
    } else if is_some_float {
        let bytes: [u8; 4] = value[..4].try_into().unwrap();
        if is_signed {
            let res_value = f32::from_le_bytes(bytes);
            let signed_value = res_value * description.divisor;

            let value = if signed_value.is_sign_negative() {
                bw.append_bit(true);
                -signed_value
            } else {
                bw.append_bit(false);
                signed_value
            };

            bw.append_u32_range(value.round() as u32, description.bits - 1);
        } else {
            let res_value = f32::from_le_bytes(bytes);
            let value = res_value * description.divisor;

            bw.append_u32_range(value.round() as u32, description.bits);
        }
    } else if is_angle {
        // Quick hack. Angle is i16 so here it is.
        let bytes: [u8; 4] = value[..4].try_into().unwrap();
        let res_value = f32::from_le_bytes(bytes);
        let multiplier = 360f32 / (1 << description.bits) as f32;
        let value = (res_value / multiplier).round() as u32;
        bw.append_u32_range(value, description.bits);
    } else if is_string {
        for c in value {
            bw.append_u8(*c);
        }
    } else {
        unreachable!("Decoded value does not match any type. Should this happens?");
    }
}

/// There's no need to add null terminator because string from the table
/// already includes it.
fn find_decoder<'a>(
    key: &'a [u8],
    delta_decoder: &'a DeltaDecoder,
) -> Option<(usize, &'a DeltaDecoderS)> {
    for (index, description) in delta_decoder.iter().enumerate() {
        if key.len() != description.name.len() {
            continue;
        }

        if description.name[..description.name.len()]
            .iter()
            .zip(key)
            .filter(|&(a, b)| a != b)
            .count()
            > 0
        {
            continue;
        }

        return Some((index, description));
    }

    None
}

/// Find delta from description name.
fn find_delta_value<'a>(name: &[u8], delta: &'a Delta) -> &'a [u8] {
    delta.get(from_utf8(name).unwrap()).unwrap()
}
