use self::types::{DeltaDecoderS, DeltaType};

use super::*;

pub fn get_initial_delta() -> DeltaDecoderTable {
    let mut res: DeltaDecoderTable = DeltaDecoderTable::new();

    let e1 = DeltaDecoderS {
        name: "flags".into(),
        bits: 32,
        divisor: 1.,
        flags: DeltaType::Integer as u32,
    };
    let e2 = DeltaDecoderS {
        name: "name".into(),
        bits: 8,
        divisor: 1.,
        flags: DeltaType::String as u32,
    };
    let e3 = DeltaDecoderS {
        name: "offset".into(),
        bits: 16,
        divisor: 1.,
        flags: DeltaType::Integer as u32,
    };
    let e4 = DeltaDecoderS {
        name: "size".into(),
        bits: 8,
        divisor: 1.,
        flags: DeltaType::Integer as u32,
    };
    let e5 = DeltaDecoderS {
        name: "bits".into(),
        bits: 8,
        divisor: 1.,
        flags: DeltaType::Integer as u32,
    };
    let e6 = DeltaDecoderS {
        name: "divisor".into(),
        bits: 32,
        divisor: 4000.,
        flags: DeltaType::Float as u32,
    };
    let e7 = DeltaDecoderS {
        name: "preMultiplier".into(),
        bits: 32,
        divisor: 4000.,
        flags: DeltaType::Float as u32,
    };

    let default_decoder = vec![e1, e2, e3, e4, e5, e6, e7];

    res.insert("delta_description_t\0".to_string(), default_decoder);

    res
}

#[macro_export]
macro_rules! nbit_num {
    ($num:expr, $bit:expr) => {{
        use dem::bit::BitWriter;

        let mut writer = BitWriter::new();
        writer.append_u32_range($num as u32, $bit);
        writer.data
    }};
}

#[macro_export]
macro_rules! nbit_str {
    ($name:expr) => {{
        use dem::bit::BitWriter;

        let mut writer = BitWriter::new();
        $name.as_bytes().iter().for_each(|s| writer.append_u8(*s));
        writer.data
    }};
}