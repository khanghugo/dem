# dem

[![0.2.0](https://img.shields.io/crates/v/dem)](https://crates.io/crates/dem) [![Documentation](https://img.shields.io/badge/docs-docs.rs-brightgreen?link=https%3A%2F%2Fdocs.rs%2Fdem%2F0.2.0%2Fdem%2F)](https://docs.rs/dem/0.2.0/dem/)

A complete GoldSrc demo parser and writer library

## Example

```rust
let mut demo = open_demo("./src/tests/demotest.dem").unwrap();

for entry in &mut demo.directory.entries {
    for frame in &mut entry.frames {
        if let FrameData::NetworkMessage(ref mut box_type) = &mut frame.frame_data {
            let data = &mut box_type.as_mut().1;
            
            if let MessageData::Parsed(messages) = &mut data.messages {
                messages.push(NetMessage::EngineMessage(Box::new(EngineMessage::SvcBad)));
            };
        }
    }
}

demo.write_to_file("./src/tests/demo2test.dem").unwrap();
```

## Acknowledgement

[hlviewer.js](https://github.com/skyrim/hlviewer.js)

[talent](https://github.com/cgdangelo/talent/tree/main)

[coldemoplayer](https://github.com/jpcy/coldemoplayer)

[hldemojs](https://github.com/Matherunner/hldemojs)