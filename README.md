# dem

A complete GoldSrc demo parser and writer library

## Example

```rust
use std::{fs::File, io::{self, Read}};

// prologue
let mut bytes = Box::new(Vec::new());
let mut f = File::open(file_name).unwrap();
f.read_to_end(&mut bytes).unwrap();

let demo = Demo::parse(&bytes).unwrap();

// do stuffs
let aux = Aux::new();

for entry in &mut demo.directory.entries {
    for frame in &mut entry.frames {
        if let FrameData::NetMsg((_, data)) = &mut frame.data {
            let (_, netmsg) = parse_netmsg(data.msg, aux.clone()).unwrap();  
            // do netmsg things  
            let bytes = write_netmsg(netmsg, aux.clone());
            data.msg = bytes.leak(); // hldemo does not own any data. Remember to free.
        }
    }
}    

// write demo
write_demo("my_new_demo", demo).unwrap();
```

## Acknowledgement

[hlviewer.js](https://github.com/skyrim/hlviewer.js)
[talent](https://github.com/cgdangelo/talent/tree/main)
[coldemoplayer](https://github.com/jpcy/coldemoplayer)
[hldemojs](https://github.com/Matherunner/hldemojs)