use bitvec::{field::BitField, order::Lsb0, slice::BitSlice as _BitSlice};

use self::types::BitVec;

use super::*;

pub type BitSlice = _BitSlice<u8, Lsb0>;

// Wraps bytes into bits because doing this with nom is a very bad idea.
pub struct BitReader<'a> {
    pub bytes: &'a BitSlice,
    // Bit offset, starting from starting of `bytes`.
    offset: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        BitReader {
            bytes: BitSlice::from_slice(bytes),
            offset: 0,
        }
    }

    pub fn read_1_bit(&mut self) -> bool {
        let res = self.bytes[self.offset];
        self.offset += 1;
        res
    }

    pub fn read_n_bit(&mut self, n: usize) -> &BitSlice {
        let range = self.offset + n;
        let res: &BitSlice = &self.bytes[self.offset..range];
        self.offset += n;
        res
    }

    /*
    char * MSG_ReadBitString(void)
    {
        uint32 uVar1;
        char *pcVar2;

        pcVar2 = MSG_ReadBitString::buf;
        MSG_ReadBitString::buf[0] = '\0';
        if (msg_badread == false) {
        do {
            uVar1 = MSG_ReadBits(8);
            if ((char)uVar1 == '\0') break;
            *pcVar2 = (char)uVar1;
            pcVar2 = pcVar2 + 1;
        } while (msg_badread == false);
        }
        *pcVar2 = '\0';
        return MSG_ReadBitString::buf;
    }
    */
    pub fn read_string(&mut self) -> &BitSlice {
        let start = self.offset;

        while self.peek_byte() != 0 {
            self.offset += 8;
        }

        // Includes the null terminator.
        self.offset += 8;

        &self.bytes[start..self.offset]
    }

    pub fn read_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut res = [0u8; N];

        for x in res.iter_mut().take(N) {
            let start = self.offset;
            let end = start + 8;

            *x = self.bytes[start..end].load_le::<u8>();

            self.offset += 8;
        }

        res
    }

    /// Peeks 8 bits and converts to u8.
    fn peek_byte(&self) -> u8 {
        self.peek_n_bits(8).to_u8()
    }

    pub fn peek_n_bits(&self, n: usize) -> &BitSlice {
        &self.bytes[self.offset..self.offset + n]
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    /// Returns the number of bits read into bytes.
    pub fn get_consumed_bytes(&self) -> usize {
        self.get_offset().div_ceil(8)
    }
}

pub struct BitWriter {
    pub data: BitVec,
}

impl Default for BitWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            data: BitVec::new(),
        }
    }

    pub fn append_bit(&mut self, i: bool) {
        self.data.push(i);
    }

    pub fn append_slice(&mut self, i: &BitSlice) {
        self.data.extend(i);
    }

    pub fn append_vec(&mut self, i: impl Into<BitVec> + AsRef<BitSlice>) {
        self.append_slice(i.as_ref())
    }

    /// Append selected bits from a u32.
    /// end = 31 means excluding the sign bit due to LE.
    pub fn append_u32_nbit(&mut self, i: u32, len: u32) {
        let start = self.data.len();
        self.data.resize(start + len as usize, false);

        let slice = &mut self.data[start..start + len as usize];

        slice.store_le::<u32>(i);
    }

    pub fn append_u2(&mut self, i: u8) {
        self.append_u32_nbit(i as u32, 2);
    }

    pub fn append_u3(&mut self, i: u8) {
        self.append_u32_nbit(i as u32, 3);
    }

    pub fn append_u4(&mut self, i: u8) {
        self.append_u32_nbit(i as u32, 4);
    }

    pub fn append_u5(&mut self, i: u8) {
        self.append_u32_nbit(i as u32, 5);
    }

    pub fn append_u6(&mut self, i: u8) {
        self.append_u32_nbit(i as u32, 6);
    }

    pub fn append_u8(&mut self, i: u8) {
        self.append_u32_nbit(i as u32, 8);
    }

    pub fn append_u9(&mut self, i: u16) {
        self.append_u32_nbit(i as u32, 9);
    }

    pub fn append_u10(&mut self, i: u16) {
        self.append_u32_nbit(i as u32, 10);
    }

    pub fn append_u11(&mut self, i: u16) {
        self.append_u32_nbit(i as u32, 11);
    }

    pub fn append_u12(&mut self, i: u16) {
        self.append_u32_nbit(i as u32, 12);
    }

    pub fn append_u16(&mut self, i: u16) {
        self.append_u32_nbit(i as u32, 16);
    }

    pub fn append_u24(&mut self, i: u32) {
        self.append_u32_nbit(i, 24);
    }

    pub fn append_u32(&mut self, i: u32) {
        self.append_u32_nbit(i, 32);
    }

    pub fn append_string(&mut self, s: impl Into<String> + AsRef<str>) {
        let s = s.as_ref();
        let bytes = s.as_bytes();

        for &byte in bytes {
            self.append_u8(byte);
        }

        // null terminator
        // don't add please
        // self.append_u8(0);
    }

    pub fn append_bytes<const N: usize>(&mut self, bytes: [u8; N]) {
        for byte in bytes {
            self.append_u8(byte);
        }
    }

    pub fn get_u8_vec(&mut self) -> Vec<u8> {
        // https://github.com/ferrilab/bitvec/issues/27
        let mut what = self.data.to_owned();
        what.force_align();
        what.into_vec()
    }
}

#[allow(dead_code)]
pub trait BitSliceCast {
    fn to_u8(&self) -> u8;
    fn to_i8(&self) -> i8;
    fn to_u16(&self) -> u16;
    fn to_i16(&self) -> i16;
    fn to_u32(&self) -> u32;
    fn to_i32(&self) -> i32;
    fn get_string(&self) -> String;
}

impl BitSliceCast for BitSlice {
    // https://github.com/ferrilab/bitvec/issues/64
    fn to_u8(&self) -> u8 {
        self.load_le::<u8>()
    }

    fn to_i8(&self) -> i8 {
        self.load_le::<i8>()
    }

    fn to_u16(&self) -> u16 {
        self.load_le::<u16>()
    }

    fn to_i16(&self) -> i16 {
        self.load_le::<i16>()
    }

    fn to_u32(&self) -> u32 {
        self.load_le::<u32>()
    }

    fn to_i32(&self) -> i32 {
        self.load_le::<i32>()
    }

    fn get_string(&self) -> String {
        let binding = self
            .chunks(8)
            .map(|chunk| chunk.to_u8())
            .collect::<Vec<u8>>();
        let s = String::from_utf8_lossy(binding.as_slice());

        s.into()
    }
}
