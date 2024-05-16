use nom::{combinator::all_consuming, multi::many0};
use types::{CustomMessage, DeltaDecoderTable, NetMessage};

use nom_helper::Result;

mod bit;
mod byte_writer;
mod delta;
pub mod netmsg_doer;
mod nom_helper;
pub mod types;
mod utils;

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
