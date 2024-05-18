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
//! let mut bytes = Box::new(Vec::new());
//! let mut f = File::open("example.dem").unwrap();
//! f.read_to_end(&mut bytes).unwrap();
//!
//! let mut demo = Demo::parse(&bytes).unwrap();
//!
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
//! write_demo("my_new_demo", demo).unwrap();
//! ```
use std::{cell::RefCell, io};

use demo_writer::DemoWriter;
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
pub mod types;

pub extern crate hldemo;

/// Auxillary data required for parsing/writing certain messages.
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

/// Writes a demo
///
/// # Example
/// ```no_run
/// use std::{fs::File, io::{self, Read}};
///
/// let mut bytes = Box::new(Vec::new());
/// let mut f = File::open(file_name).unwrap();
/// f.read_to_end(&mut bytes).unwrap();
///
/// let demo = Demo::parse(&bytes).unwrap();
///
/// // do your stuffs
///
/// write_demo("my_new_demo", demo).unwrap();
/// ```
pub fn write_demo(file_name: &str, demo: Demo) -> io::Result<()> {
    let mut out = DemoWriter::new(String::from(file_name));

    out.write_file(demo)?;

    Ok(())
}
