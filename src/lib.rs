//! GoldSrc demo parser and writer
//!
//! # Example
//!
//! ```ignore
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
use std::{ffi::OsStr, fs::OpenOptions, io::Read, path::Path};

use types::{DeltaDecoderTable, Demo};

mod byte_writer;
mod delta;
mod nom_helper;
mod utils;

pub mod bit;
pub mod demo_parser;
pub mod demo_writer;
pub mod error;
pub mod netmsg_doer;
pub mod types;

// need this to have the conversion function
pub use crate::bit::BitSliceCast;

use crate::{demo_parser::parse_demo, error::DemoError, types::MessageDataParseMode};
pub use utils::bitslice_to_string;

// /// Re-exporting hldemo to have latest changes than 0.3.0 hldemo
// pub extern crate hldemo;

/// Re-exporting bitvec to avoid clogging the main project
pub extern crate bitvec;

impl Demo {
    /// It is faster to parse netmessage because it just is for some reasons
    pub fn parse_from_file(
        path: impl AsRef<OsStr> + AsRef<Path>,
        netmsg_parse_mode: MessageDataParseMode,
    ) -> Result<Self, DemoError> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut bytes: Vec<u8> = vec![];

        file.read_to_end(&mut bytes)?;

        Self::parse_from_bytes(bytes.as_slice(), netmsg_parse_mode)
    }

    pub fn parse_from_bytes(
        demo_bytes: &[u8],
        netmsg_parse_mode: MessageDataParseMode,
    ) -> Result<Self, DemoError> {
        parse_demo(demo_bytes, netmsg_parse_mode)
            // discard errors because they aren't very helpful anyway
            .map_err(|_| DemoError::ParseError)
            .map(|(_, x)| x)
    }
}

/// Opens a demo
///
/// # Example
/// ```ignore
/// let demo = open_demo("./tests/demotest.dem").unwrap();
/// ```
pub fn open_demo(demo_path: impl AsRef<Path> + AsRef<OsStr>) -> Result<Demo, DemoError> {
    Demo::parse_from_file(demo_path, types::MessageDataParseMode::Parse)
}

pub fn open_demo_from_bytes(demo_bytes: &[u8]) -> Result<Demo, DemoError> {
    Demo::parse_from_bytes(demo_bytes, types::MessageDataParseMode::Parse)
}

/// Writes a [`u32`] into [`types::BitVec`]
///
/// (number: u32, bit_count)
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
    fn write_read() {
        let mut dem = open_demo("./src/tests/demotest.dem").unwrap();
        let a = dem.write_to_bytes();
        let _dem = open_demo_from_bytes(&a).unwrap();
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

                    // let a = open_demo(path.as_path()).unwrap();

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
