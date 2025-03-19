//! GoldSrc demo parser and writer
//!
//! # Example
//!
//! ```no_run
//! let mut demo = open_demo("./src/tests/demotest.dem").unwrap();
//!
//! for entry in &mut demo.directory.entries {
//!     for frame in &mut entry.frames {
//!         if let FrameData::NetworkMessage(ref mut box_type) = &mut frame.frame_data {
//!             let data = &mut box_type.as_mut().1;
//!             
//!             if let MessageData::Parsed(messages) = &mut data.messages {
//!                 messages.push(NetMessage::EngineMessage(Box::new(EngineMessage::SvcBad)));
//!             };
//!         }
//!     }
//! }
//!
//! demo.write_to_file("./src/tests/demo2test.dem").unwrap();
//! ```
use std::{ffi::OsStr, path::Path};

use nom::{combinator::all_consuming, multi::many0};
use types::{AuxRefCell, ByteVec, DeltaDecoderTable, Demo, NetMessage};

use nom_helper::Result;

mod byte_writer;
mod delta;
mod nom_helper;
mod utils;

pub mod bit;
pub mod demo_parser;
pub mod demo_writer;
pub mod netmsg_doer;
pub mod prelude;
pub mod types;

pub use utils::bitslice_to_string;

// /// Re-exporting hldemo to have latest changes than 0.3.0 hldemo
// pub extern crate hldemo;

/// Re-exporting bitvec to avoid clogging the main project
pub extern crate bitvec;

/// Parses all bytes in `data.msg` for each demo frame.
///
///
pub fn parse_netmsg(i: &[u8], aux: AuxRefCell) -> Result<Vec<NetMessage>> {
    let parser = move |i| NetMessage::parse(i, aux.clone());
    all_consuming(many0(parser))(i)
}

/// Should be used for replacing `data.msg` of each frame.
pub fn write_netmsg(i: &Vec<NetMessage>, aux: AuxRefCell) -> ByteVec {
    let mut res: ByteVec = vec![];

    for message in i {
        res.append(&mut message.write(aux.clone()))
    }

    res
}

/// Opens a demo
///
/// # Example
/// ```no_run
/// let demo = open_demo("./tests/demotest.dem").unwrap();
/// ```
pub fn open_demo(demo_path: impl AsRef<Path> + AsRef<OsStr>) -> eyre::Result<Demo> {
    Demo::parse_from_file(demo_path, types::MessageDataParseMode::Parse)
}

pub fn open_demo_from_bytes(demo_bytes: &[u8]) -> eyre::Result<Demo> {
    Demo::parse_from_bytes(demo_bytes, types::MessageDataParseMode::Parse)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn open() {
        open_demo("./src/tests/demotest.dem").unwrap();
    }

    #[test]
    fn open_without_netmessage() {
        Demo::parse_from_file(
            "./src/tests/demotest.dem",
            types::MessageDataParseMode::Parse,
        )
        .unwrap();
    }

    #[test]
    fn write() {
        let dem = open_demo("./src/tests/demotest.dem").unwrap();
        dem.write_to_file("./src/tests/demotest_out.dem").unwrap();
    }

    #[test]
    fn read_a_lot() {
        let folder = "./src/tests/";

        std::fs::read_dir(folder)
            .map(|res| {
                res.filter_map(|entry| entry.ok()).for_each(|entry| {
                    let path = entry.path();

                    if path.is_dir() {
                        return;
                    }

                    if path.extension().map(|ext| ext != "dem").unwrap_or(false) {
                        return;
                    }

                    assert!(
                        open_demo(path.as_path()).is_ok(),
                        "error opening {}",
                        path.display()
                    )
                })
            })
            .unwrap_or_else(|_| assert!(false));
    }
}
