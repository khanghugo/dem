//! GoldSrc demo parser and writer
//!
//! # Example
//!
//! ```no_run
//! use dem::Aux;
//! use dem::{parse_netmsg, write_demo, write_netmsg};
//! use dem::hldemo::Demo;
//! use dem::hldemo::FrameData;
//! use std::{fs::File, io::Read, cell::RefCell};
//!
//! // prologue
//! let demo = open_demo!("example.dem");
//! // do stuffs
//! let aux = Aux::new();
//!
//! for entry in &mut demo.directory.entries {
//!     for frame in &mut entry.frames {
//!         if let FrameData::NetMsg((_, data)) = &mut frame.data {
//!             let (_, netmsg) = parse_netmsg(data.msg, &aux).unwrap();
//!             // do netmsg things
//!             let bytes = write_netmsg(netmsg, &aux);
//!             data.msg = bytes.leak(); // hldemo does not own any data. Remember to free.
//!         }
//!     }
//! }
//!
//! // write demo
//! write_demo!("my_new_demo", demo).unwrap();
//! ```
use std::{
    cell::RefCell,
    fs::OpenOptions,
    io::{self, Read},
    path::{Path, PathBuf},
};

use demo_writer::DemoWriter;
use eyre::eyre;
use hldemo::Demo;
use nom::{combinator::all_consuming, multi::many0};
use types::{ByteVec, CustomMessage, DeltaDecoderTable, NetMessage};

use nom_helper::Result;
use utils::get_initial_delta;

mod byte_writer;
mod delta;
mod nom_helper;
mod utils;

pub mod bit;
pub mod demo_writer;
pub mod netmsg_doer;
pub mod prelude;
pub mod types;

pub use utils::bitslice_to_string;

/// Re-exporting hldemo to have latest changes than 0.3.0 hldemo
pub extern crate hldemo;

/// Re-exporting bitvec to avoid clogging the main project
pub extern crate bitvec;

/// Auxillary data required for parsing/writing certain messages.
///
/// This includes delta decoders, custom messages, and max client
#[derive(Debug)]
pub struct Aux {
    delta_decoders: DeltaDecoderTable,
    max_client: u8,
    custom_messages: CustomMessage,
}

impl Aux {
    pub fn new() -> RefCell<Self> {
        RefCell::new(Self {
            delta_decoders: get_initial_delta(),
            max_client: 1,
            custom_messages: CustomMessage::new(),
        })
    }
}

/// Parses all bytes in `data.msg` for each demo frame.
///
/// Must be invoked for individual frames.
pub fn parse_netmsg<'a>(i: &'a [u8], aux: &'a RefCell<Aux>) -> Result<'a, Vec<NetMessage>> {
    let parser = move |i| NetMessage::parse(i, aux);
    all_consuming(many0(parser))(i)
}

/// Should be used for replacing `data.msg` of each frame.
pub fn write_netmsg(i: Vec<NetMessage>, aux: &RefCell<Aux>) -> ByteVec {
    let mut res: ByteVec = vec![];

    for message in i {
        res.append(&mut message.write(aux))
    }

    res
}

/// Opens a demo
///
/// # Example
/// ```no_run
/// let demo = open_demo("./tests/demotest.dem").unwrap();
/// ```
pub fn open_demo(demo_path: impl AsRef<Path> + Into<PathBuf>) -> eyre::Result<Demo<'static>> {
    let mut bytes = Vec::new();

    let mut file = OpenOptions::new().read(true).open(demo_path)?;
    file.read_to_end(&mut bytes)?;

    open_demo_from_bytes(bytes.leak())
}

pub fn open_demo_from_bytes(bytes: &'static [u8]) -> eyre::Result<Demo<'static>> {
    match hldemo::Demo::parse(bytes) {
        Ok(demo) => Ok(demo),
        Err(_) => Err(eyre!("cannot parse demo")),
    }
}

/// Writes a demo
///
/// # Example
/// ```no_run
/// let demo = open_demo("./tests/demotest.dem").unwrap();
/// // do your stuffs
/// write_demo("my_new_demo", demo).unwrap();
/// ```
///
/// ### Warning, for the time being, the function would consume the demo so I can avoid doing lifetime stuffs (all because hldemo does not own data)
pub fn write_demo(demo_path: impl AsRef<Path> + Into<PathBuf>, demo: Demo) -> eyre::Result<()> {
    let mut out = DemoWriter::new(demo_path);

    out.write_file(demo)?;

    Ok(())
}

/// Writes a [`u32`] into [`types::BitVec`]
#[macro_export]
macro_rules! nbit_num {
    ($num:expr, $bit:expr) => {{
        use $crate::bit::BitWriter;

        let mut writer = BitWriter::new();
        writer.append_u32_range($num as u32, $bit);
        writer.data
    }};
}

/// Writes a string into [`types::BitVec`]
#[macro_export]
macro_rules! nbit_str {
    ($name:expr) => {{
        use $crate::bit::BitWriter;

        let mut writer = BitWriter::new();
        $name.as_bytes().iter().for_each(|s| writer.append_u8(*s));
        writer.data
    }};
}

/// Parses through the first entry for auxillary data
#[macro_export]
macro_rules! init_parse {
    ($demo:ident) => {{
        use $crate::parse_netmsg;
        use $crate::Aux;

        let mut aux = Aux::new();

        $demo
            .directory
            .entries
            .get(0)
            .unwrap()
            .frames
            .iter()
            .for_each(|frame| match &frame.data {
                FrameData::NetMsg((_, data)) => {
                    parse_netmsg(data.msg, &aux).unwrap();
                }
                _ => (),
            });

        aux
    }};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn open() {
        open_demo("./tests/demotest.dem").unwrap();
    }

    #[test]
    fn write() {
        let dem = open_demo("./tests/demotest.dem").unwrap();
        let _res = write_demo("out.dem", dem).unwrap();
    }
}
