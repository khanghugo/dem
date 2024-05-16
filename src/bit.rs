use bitvec::{field::BitField, order::Lsb0, slice::BitSlice as _BitSlice};

use self::types::BitVec;

use super::*;

pub type BitSlice = _BitSlice<u8, Lsb0>;

// Wraps bytes into bits because doing this with nom is a very bad idea.
pub struct BitReader {
    pub bytes: BitVec,
    // Bit offset, starting from starting of `bytes`.
    offset: usize,
}

impl BitReader {
    pub fn new(bytes: &[u8]) -> Self {
        BitReader {
            bytes: BitVec::from_slice(bytes),
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

    pub fn read_string(&mut self) -> &BitSlice {
        let start = self.offset;

        // The second condition is to make sure we are aligned.
        while self.peek_byte() != 0 || (self.offset - start) % 8 != 0 {
            self.offset += 1;
        }

        // Includes the null terminator.
        self.offset += 8;

        &self.bytes[start..self.offset]
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
        let current_bit = self.get_offset();
        let modulo = current_bit % 8;
        let remaining_bits = if modulo == 0 { 0 } else { 8 - modulo };

        (current_bit + remaining_bits) / 8
    }
}

pub struct BitWriter {
    pub data: BitVec,
    pub offset: usize,
}

#[allow(dead_code)]
impl BitWriter {
    pub fn new() -> Self {
        Self {
            data: BitVec::new(),
            offset: 0,
        }
    }

    fn offset(&mut self, i: usize) {
        self.offset += i;
    }

    pub fn append_bit(&mut self, i: bool) {
        self.data.push(i);
        self.offset(1);
    }

    pub fn append_slice(&mut self, i: &BitSlice) {
        self.data.extend(i);
        self.offset(i.len());
    }

    pub fn append_vec(&mut self, i: &BitVec) {
        self.append_slice(i.as_bitslice())
    }

    pub fn append_u8(&mut self, i: u8) {
        let bits: BitVec = BitVec::from_element(i);
        self.append_vec(&bits);
    }

    /// Append selected bits from a u32.
    /// end = 31 means excluding the sign bit due to LE.
    pub fn append_u32_range(&mut self, i: u32, end: u32) {
        let bits: BitVec = i
            .to_le_bytes()
            .iter()
            .flat_map(|byte| BitVec::from_element(*byte))
            .collect();
        self.append_slice(&bits[..end as usize]);
    }

    pub fn append_i32_range(&mut self, i: i32, end: u32) {
        let bits: BitVec = i
            .to_le_bytes()
            .iter()
            .flat_map(|byte| BitVec::from_element(*byte))
            .collect();
        self.append_slice(&bits[..end as usize]);
    }

    pub fn insert_bit(&mut self, i: bool, pos: usize) {
        self.data.insert(pos, i);
        self.offset(1);
    }

    pub fn insert_slice(&mut self, i: &BitSlice, pos: usize) {
        for (offset, what) in i.iter().enumerate() {
            self.insert_bit(*what, pos + offset);
        }
    }

    pub fn insert_vec(&mut self, i: BitVec, pos: usize) {
        self.insert_slice(i.as_bitslice(), pos);
    }

    pub fn insert_u8(&mut self, i: u8, pos: usize) {
        let bits: BitVec = BitVec::from_element(i);
        self.insert_slice(&bits, pos);
    }

    pub fn insert_u32_range(&mut self, i: u32, end: u32, pos: usize) {
        let bits: BitVec = i
            .to_le_bytes()
            .iter()
            .flat_map(|byte| BitVec::from_element(*byte))
            .collect();

        self.insert_slice(&bits[..end as usize], pos);
    }

    pub fn get_u8_vec(&mut self) -> Vec<u8> {
        // https://github.com/ferrilab/bitvec/issues/27
        let mut what = self.data.to_owned();
        what.force_align();
        what.into_vec()
    }

    pub fn get_offset(&self) -> usize {
        self.offset
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
}

impl BitSliceCast for BitSlice {
    // https://github.com/ferrilab/bitvec/issues/64
    fn to_u8(&self) -> u8 {
        self.load::<u8>()
    }

    fn to_i8(&self) -> i8 {
        self.load::<i8>()
    }

    fn to_u16(&self) -> u16 {
        self.load::<u16>()
    }

    fn to_i16(&self) -> i16 {
        self.load::<i16>()
    }

    fn to_u32(&self) -> u32 {
        self.load::<u32>()
    }

    fn to_i32(&self) -> i32 {
        self.load::<i32>()
    }
}
