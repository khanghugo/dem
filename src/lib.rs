//! GoldSrc demo parser and writer
//!
//! # Example
//!
//! ```no_run
//! use std::{fs::File, io::{self, Read}};
//!
//! // prologue
//! let mut bytes = Box::new(Vec::new());
//! let mut f = File::open(file_name).unwrap();
//! f.read_to_end(&mut bytes).unwrap();
//!
//! let demo = Demo::parse(&bytes).unwrap();
//!
//! // do stuffs
//! let aux = Aux::new();
//!
//! for entry in &mut demo.directory.entries {
//!     for frame in &mut entry.frames {
//!         if let FrameData::NetMsg((_, data)) = &mut frame.data {
//!             let (_, netmsg) = parse_netmsg(data.msg, aux.clone()).unwrap();  
//!             // do netmsg things  
//!             let bytes = write_netmsg(netmsg, aux.clone());
//!             data.msg = bytes.leak(); // hldemo does not own any data. Remember to free.
//!         }
//!     }
//! }    
//!
//! // write demo
//! write_demo("my_new_demo", demo).unwrap();
//! ```
use std::io;

use demo_writer::DemoWriter;
use hldemo::Demo;
use nom::{combinator::all_consuming, multi::many0};
use types::{ByteVec, CustomMessage, DeltaDecoderTable, NetMessage};

use nom_helper::Result;
use utils::get_initial_delta;

mod bit;
mod byte_writer;
mod delta;
mod demo_writer;
mod nom_helper;
mod utils;

mod netmsg_doer;
pub mod types;

/// Auxillary data required for parsing/writing certain messages.
#[derive(Clone)]
pub struct Aux {
    delta_decoders: Box<DeltaDecoderTable>,
    max_client: u8,
    custom_messages: Box<CustomMessage>,
}

impl Aux {
    pub fn new() -> Self {
        Self {
            delta_decoders: Box::new(get_initial_delta()),
            max_client: 1,
            custom_messages: Box::new(CustomMessage::new()),
        }
    }
}

impl Default for Aux {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses all bytes in `data.msg` for each demo frame.
///
/// Must be invoked for individual frames.
pub fn parse_netmsg(i: &[u8], aux: Aux) -> Result<Vec<NetMessage>> {
    // Cloning pointer so it should be good
    let parser = move |i| NetMessage::parse(i, aux.clone());
    all_consuming(many0(parser))(i)
}

/// Should be used for replacing `data.msg` of each frame.
pub fn write_netmsg(i: Vec<NetMessage>, aux: Aux) -> ByteVec {
    let mut res: ByteVec = vec![];

    for message in i {
        res.append(&mut message.write(aux.clone()))
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
