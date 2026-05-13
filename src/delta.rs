use crate::{
    bit::{BitReader, BitSliceCast, BitWriter},
    types::{Delta, DeltaDecoder, DeltaDecoderS, DeltaType, DeltaValue},
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
                let value = parse_delta_field(description, br);
                res.insert(description.name.to_owned(), value);
            }
        }
    }

    res
}

macro_rules! flag {
    ($lhs:expr, $rhs:expr) => {{ $lhs as u32 & $rhs as u32 != 0 }};
}

fn parse_delta_field(description: &DeltaDecoderS, br: &mut BitReader) -> DeltaValue {
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
            let res_value = (sign * value as i8) / description.divisor as i8;

            DeltaValue::ByteSigned(res_value)
        } else {
            let value = (br.read_n_bit(description.bits as usize)).to_u8();
            let res_value = value / description.divisor as u8;

            DeltaValue::ByteUnsigned(res_value)
        }
    } else if is_short {
        if is_signed {
            let sign = if br.read_1_bit() { -1 } else { 1 };
            let value = (br.read_n_bit(description.bits as usize - 1)).to_u16();
            let res_value = (sign * value as i16) / description.divisor as i16;

            DeltaValue::ShortSigned(res_value)
        } else {
            let value = (br.read_n_bit(description.bits as usize)).to_u16();
            let res_value = value / description.divisor as u16;

            DeltaValue::ShortUnsigned(res_value)
        }
    } else if is_integer {
        if is_signed {
            let sign = if br.read_1_bit() { -1 } else { 1 };
            let value = (br.read_n_bit(description.bits as usize - 1)).to_u32();
            let res_value = (sign * value as i32) / description.divisor as i32;

            DeltaValue::IntSigned(res_value)
        } else {
            let value = (br.read_n_bit(description.bits as usize)).to_u32();
            let res_value = value / description.divisor as u32;

            DeltaValue::IntUnsigned(res_value)
        }
    } else if is_some_float {
        if is_signed {
            let sign = if br.read_1_bit() { -1 } else { 1 };
            let value = (br.read_n_bit(description.bits as usize - 1)).to_u32();

            DeltaValue::FloatSigned((sign * value as i32) as f32 / description.divisor)
        } else {
            let value = (br.read_n_bit(description.bits as usize)).to_u32();

            DeltaValue::FloatUnsigned(value as f32 / description.divisor)
        }
    } else if is_angle {
        let value = (br.read_n_bit(description.bits as usize)).to_u32();
        let multiplier = 360f32 / ((1 << description.bits) as f32);
        let res_value = value as f32 * multiplier;

        DeltaValue::Angle(res_value)
    } else if is_string {
        DeltaValue::String(br.read_string().get_string())
    } else {
        unreachable!("Encoded value does not match any types. Should this happens?");
    }
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
        let (index, _) = delta_decoder
            .iter()
            .enumerate()
            .find(|(_, x)| x.name == key.as_str())
            .unwrap();

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

    bw.append_u3(byte_mask_count);
    for i in 0..byte_mask_count {
        bw.append_u8(byte_mask[i as usize]);
    }

    // We have to write delta by the described order.
    for description in delta_decoder {
        if delta.contains_key(&description.name) {
            write_delta_field(description, delta.get(&description.name).unwrap(), bw);
        }
    }
}

fn write_delta_field(description: &DeltaDecoderS, value: &DeltaValue, bw: &mut BitWriter) {
    // delta writing does not need to care about flags
    match value {
        DeltaValue::ByteSigned(x) => {
            let res = x * description.divisor as i8;

            bw.append_bit(x.is_negative());
            bw.append_u32_nbit(res.unsigned_abs() as u32, description.bits - 1);
        }
        DeltaValue::ByteUnsigned(x) => {
            // FIXME: it is possible that divisor could have decimals
            // and then this broke
            // I think the actual correct way is to cast x to f32 then do math
            // and then cast it back
            // but no delta files use decimal devisor, so all good for now
            bw.append_u32_nbit((x * description.divisor as u8) as u32, description.bits);
        }
        DeltaValue::ShortSigned(x) => {
            let res = x * description.divisor as i16;

            bw.append_bit(x.is_negative());
            bw.append_u32_nbit(res.unsigned_abs() as u32, description.bits - 1);
        }
        DeltaValue::ShortUnsigned(x) => {
            bw.append_u32_nbit((x * description.divisor as u16) as u32, description.bits);
        }
        DeltaValue::IntSigned(x) => {
            let res = x * description.divisor as i32;

            bw.append_bit(x.is_negative());
            bw.append_u32_nbit(res.unsigned_abs(), description.bits - 1);
        }
        DeltaValue::IntUnsigned(x) => {
            bw.append_u32_nbit(x * description.divisor as u32, description.bits);
        }
        DeltaValue::FloatSigned(x) => {
            let res = x * description.divisor;

            bw.append_bit(res.is_sign_negative());
            bw.append_u32_nbit(res.abs().round() as u32, description.bits - 1);
        }
        DeltaValue::FloatUnsigned(x) => {
            bw.append_u32_nbit((x * description.divisor).round() as u32, description.bits);
        }
        DeltaValue::Angle(x) => {
            let multiplier = 360f32 / (1 << description.bits) as f32;
            bw.append_u32_nbit((x / multiplier).round() as u32, description.bits);
        }
        DeltaValue::String(x) => {
            bw.append_string(x);
        }
    }
}
