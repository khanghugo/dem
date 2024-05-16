use nom::{combinator::all_consuming, multi::many0};
use types::{CustomMessage, DeltaDecoderTable, NetMessage};

use nom_helper::Result;

mod bit;
mod byte_writer;
mod delta;
mod demo_writer;
mod nom_helper;
mod utils;

pub mod netmsg_doer;
pub mod types;

/// Auxillary data required for parsing/writing certain messages.
#[derive(Clone)]
pub struct Aux {
    delta_decoders: Box<DeltaDecoderTable>,
    max_client: u8,
    custom_messages: Box<CustomMessage>,
}

pub fn parse_netmsg(i: &[u8], aux: Aux) -> Result<Vec<NetMessage>> {
    // Cloning pointer so it should be good
    let parser = move |i| NetMessage::parse(i, aux.clone());
    all_consuming(many0(parser))(i)
}

#[macro_export]
macro_rules! open_demo {
    ($name:literal) => {{
        let mut bytes = Vec::new();
        let mut f = File::open($name).unwrap();
        f.read_to_end(&mut bytes).unwrap();

        hldemo::Demo::parse(bytes.leak()).unwrap()
    }};

    ($name:ident) => {{
        let mut bytes = Vec::new();
        let mut f = File::open($name).unwrap();
        f.read_to_end(&mut bytes).unwrap();

        hldemo::Demo::parse(bytes.leak()).unwrap()
    }};
}

#[macro_export]
macro_rules! write_demo {
    ($demo_name:literal, $demo:ident) => {{
        use demosuperimpose_goldsrc::writer::DemoWriter;
        let mut out = DemoWriter::new(String::from($demo_name));
        out.write_file($demo);
    }};

    ($demo_name:ident, $demo:ident) => {{
        use demosuperimpose_goldsrc::writer::DemoWriter;
        let mut out = DemoWriter::new(String::from($demo_name));
        out.write_file($demo);
    }};
}
