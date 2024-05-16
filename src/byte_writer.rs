pub struct ByteWriter {
    pub data: Vec<u8>,
    // Offset isn't really needed because we do vector and we can find offset easily.
    // But all in the spirit of 🚀.
    offset: usize,
}

impl ByteWriter {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
        }
    }

    fn offset(&mut self, offset: usize) {
        self.offset += offset;
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn append_u32(&mut self, i: u32) {
        self.data.extend(i.to_le_bytes());
        self.offset(4);
    }

    pub fn append_i32(&mut self, i: i32) {
        self.data.extend(i.to_le_bytes());
        self.offset(4);
    }

    pub fn append_f32(&mut self, i: f32) {
        self.data.extend(i.to_le_bytes());
        self.offset(4);
    }

    pub fn append_u8(&mut self, i: u8) {
        self.data.extend(i.to_le_bytes());
        self.offset(1);
    }

    pub fn append_i8(&mut self, i: i8) {
        self.data.extend(i.to_le_bytes());
        self.offset(1);
    }

    pub fn append_u16(&mut self, i: u16) {
        self.data.extend(i.to_le_bytes());
        self.offset(2);
    }

    pub fn append_i16(&mut self, i: i16) {
        self.data.extend(i.to_le_bytes());
        self.offset(2);
    }

    pub fn append_u8_slice(&mut self, i: &[u8]) {
        self.data.extend_from_slice(i);
        self.offset(i.len());
    }

    pub fn append_i16_slice(&mut self, i: &[i16]) {
        for what in i {
            self.append_i16(*what);
        }
    }

    pub fn append_f32_array(&mut self, i: [f32; 3]) {
        i.iter().for_each(|num| self.append_f32(*num));
    }

    pub fn append_i32_array_4(&mut self, i: [i32; 4]) {
        i.iter().for_each(|num| self.append_i32(*num));
    }
}
