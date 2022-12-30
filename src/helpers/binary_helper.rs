#![allow(dead_code)]
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Error, ErrorKind};

#[doc(hidden)]
macro_rules! read_integer {
    ($name:ident, $ty: ty, $bytes: expr) => {
        pub fn $name(&mut self) -> std::io::Result<$ty> {
            let mut data = self.read_bytes($bytes)?;
            data.$name::<LittleEndian>()
        }
    };

    ($name:ident, $ty: ty) => {
        pub fn $name(&mut self) -> std::io::Result<$ty> {
            let mut data = self.read_bytes(1)?;
            data.$name()
        }
    };
}

pub struct BinaryHelper {
    data: Vec<u8>,
    pos: usize,
    length: usize,
}

impl BinaryHelper {
    fn initialize() -> Self {
        Self {
            data: Vec::new(),
            pos: 0,
            length: 0,
        }
    }

    pub fn from_u8(get: &[u8]) -> Self {
        let mut a = Self::initialize();
        a.data = get.to_vec();
        a.length = get.len();
        a
    }

    pub fn remaining_length(&self) -> usize {
        self.length - self.pos
    }

    pub fn adv(&mut self, size: usize) {
        self.pos += size
    }

    pub fn read(&mut self, size: usize) -> Option<&[u8]> {
        let data = self.data.get(self.pos..self.pos + size);
        self.pos += size;
        data
    }

    pub fn read_bytes(&mut self, bytes: usize) -> std::io::Result<&[u8]> {
        let data = self.data.get(self.pos..self.pos + bytes).ok_or_else(|| {
            Error::new(
                ErrorKind::UnexpectedEof,
                format!("failed to read {} bytes from offset {}", bytes, self.pos),
            )
        })?;
        self.pos += bytes;

        Ok(data)
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_7bit_encoded_int();
        let vec = self.read(length as usize).unwrap();
        std::str::from_utf8(vec).unwrap().to_string()
    }

    pub fn read_7bit_encoded_int(&mut self) -> u8 {
        let mut count: u8 = 0;
        let mut shift: i32 = 0;
        let b: u8 = self.data[self.pos];
        loop {
            count |= (b & 0x7F) << shift;
            shift += 7;
            self.adv(1);

            if (b & 0x80) == 0 {
                break;
            };
        }
        count
    }

    read_integer!(read_i8, i8);
    read_integer!(read_i16, i16, 2);
    read_integer!(read_i24, i32, 3);
    read_integer!(read_i32, i32, 4);
    read_integer!(read_i48, i64, 6);
    read_integer!(read_i64, i64, 8);
    read_integer!(read_i128, i128, 16);

    read_integer!(read_u8, u8);
    read_integer!(read_u16, u16, 2);
    read_integer!(read_u24, u32, 3);
    read_integer!(read_u32, u32, 4);
    read_integer!(read_u48, u64, 6);
    read_integer!(read_u64, u64, 8);
    read_integer!(read_u128, u128, 16);

    read_integer!(read_f32, f32, 4);
    read_integer!(read_f64, f64, 8);
}
