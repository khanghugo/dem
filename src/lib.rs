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
pub struct Aux {
    delta_decoders: DeltaDecoderTable,
    max_client: u8,
    custom_messages: CustomMessage,
}

pub fn parse_netmsg(i: &[u8], aux: Aux) -> Result<Vec<NetMessage>> {
    let parser = move |i| NetMessage::parse(i, aux);
    all_consuming(many0(parser))(i)
}
