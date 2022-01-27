#![allow(unused_imports, unused_variables)]

use core::mem;
use std::{fmt, io};
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::result::Result;

use crate::error::ByteBufError;

pub mod slice_util;

const CHUNK_SIZE: usize = 1024;

pub type ByteReult<T> = Result<T, ByteBufError>;

pub struct ByteBuf {
    buf: Vec<u8>,
    capacity: usize,
    read_mark: isize,
    write_mark: isize,
    read_index: usize,
    write_index: usize,
}

impl Deref for ByteBuf {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        return &self.buf;
    }
}

impl DerefMut for ByteBuf {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        return &mut self.buf;
    }
}


impl Debug for ByteBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.buf.as_slice())
    }
}


impl ByteBuf {
    pub fn new_with_capacity(mut capacity: usize) -> Self {
        if capacity < 1 {
            capacity = CHUNK_SIZE
        }
        ByteBuf {
            buf: Vec::with_capacity(capacity),
            capacity,
            read_mark: -1,
            write_mark: -1,
            read_index: 0,
            write_index: 0,

        }
    }

    pub fn new_from(src: &[u8]) -> Self {
        ByteBuf {
            buf: Vec::from(src),
            capacity: src.len(),
            read_mark: -1,
            write_mark: -1,
            read_index: 0,
            write_index: if src.len() == 0 { 0 } else { src.len() },
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn clone(&self) -> ByteBuf {
        let ByteBuf {
            buf, capacity, read_mark, write_mark, read_index, write_index
        } = self;
        ByteBuf {
            buf: buf.clone(),
            capacity: *capacity,
            read_mark: -1,
            write_mark: -1,
            read_index: *read_index,
            write_index: *write_index,
        }
    }


    pub fn deep_clone(&mut self) -> ByteBuf {
        let mut buf = &mut ByteBuf {
            buf: vec![0u8; self.available_bytes().len() as usize],
            capacity: self.capacity,
            read_mark: -1,
            write_mark: -1,
            read_index: 0,
            write_index: self.write_index,
        };
        let mut bytes = self.buf.as_mut_slice();
        buf.copy_from_slice(bytes, 0);
        buf.clone()
    }


    fn copy_from_slice(&mut self, src: &mut [u8], dest_start: usize) {
        unsafe {
            let s_ptr = src.as_mut_ptr();
            let src_ptr = s_ptr.add(0);
            let d_ptr = self.buf.as_mut_ptr();
            let dest_ptr = d_ptr.add(dest_start);
            ptr::copy(src_ptr, dest_ptr, src.len());
        }
    }


    // set methods

    pub fn set_u8_be(&mut self, wid: usize, v: u8) -> ByteReult<u8> {
        if (mem::size_of::<u8>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_u8_le(&mut self, wid: usize, v: u8) -> ByteReult<u8> {
        if (mem::size_of::<u8>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }

    pub fn set_i8_be(&mut self, wid: usize, v: i8) -> ByteReult<i8> {
        if (mem::size_of::<i8>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_i8_le(&mut self, wid: usize, v: i8) -> ByteReult<i8> {
        if (mem::size_of::<i8>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }


    pub fn set_u16_be(&mut self, wid: usize, v: u16) -> ByteReult<u16> {
        if (mem::size_of::<u16>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_u16_le(&mut self, wid: usize, v: u16) -> ByteReult<u16> {
        if (mem::size_of::<u16>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }


    pub fn set_i16_be(&mut self, wid: usize, v: i16) -> ByteReult<i16> {
        if (mem::size_of::<i16>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }


    pub fn set_i16_le(&mut self, wid: usize, v: i16) -> ByteReult<i16> {
        if (mem::size_of::<i16>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }


    pub fn set_u32_be(&mut self, wid: usize, v: u32) -> ByteReult<u32> {
        if (mem::size_of::<u32>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_u32_le(&mut self, wid: usize, v: u32) -> ByteReult<u32> {
        if (mem::size_of::<u32>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }


    pub fn set_i32_be(&mut self, wid: usize, v: i32) -> ByteReult<i32> {
        if (mem::size_of::<i32>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_i32_le(&mut self, wid: usize, v: i32) -> ByteReult<i32> {
        if (mem::size_of::<i32>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }


    pub fn set_u64_be(&mut self, wid: usize, v: u64) -> ByteReult<u64> {
        if (mem::size_of::<u64>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_u64_le(&mut self, wid: usize, v: u64) -> ByteReult<u64> {
        if (mem::size_of::<u64>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }


    pub fn set_i64_be(&mut self, wid: usize, v: i64) -> ByteReult<i64> {
        if (mem::size_of::<i64>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }


    pub fn set_i64_le(&mut self, wid: usize, v: i64) -> ByteReult<i64> {
        if (mem::size_of::<i64>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }


    pub fn set_u128_be(&mut self, wid: usize, v: u128) -> ByteReult<u128> {
        if (mem::size_of::<u128>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_u128_le(&mut self, wid: usize, v: u128) -> ByteReult<u128> {
        if (mem::size_of::<u128>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }

    pub fn set_i128_be(&mut self, wid: usize, v: i128) -> ByteReult<i128> {
        if (mem::size_of::<i128>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_i128_le(&mut self, wid: usize, v: i128) -> ByteReult<i128> {
        if (mem::size_of::<i128>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }

    pub fn set_f32_be(&mut self, wid: usize, v: f32) -> ByteReult<f32> {
        if (mem::size_of::<f32>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_f32_le(&mut self, wid: usize, v: f32) -> ByteReult<f32> {
        if (mem::size_of::<f32>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }

    pub fn set_f64_be(&mut self, wid: usize, v: f64) -> ByteReult<f64> {
        if (mem::size_of::<f64>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_be_bytes(), wid);
        Ok(v)
    }

    pub fn set_f64_le(&mut self, wid: usize, v: f64) -> ByteReult<f64> {
        if (mem::size_of::<f64>() + wid) > self.capacity {
            return Err(ByteBufError::new("wid out of capacity".to_string()));
        }
        self.copy_from_slice(&mut v.to_le_bytes(), wid);
        Ok(v)
    }

    // set end


    pub fn get_bytes(&self, bytes: &mut [u8]) -> usize {
        let len = bytes.len();
        bytes.copy_from_slice(&self.buf[self.read_index..self.read_index + len]);
        bytes.len()
    }

    pub fn get_bool(&self) -> bool {
        let v = self.buf.as_slice()[self.read_index..self.read_index + mem::size_of::<bool>()].first().unwrap();
        if *v == 0x01u8 {
            true
        } else {
            false
        }
    }


    pub fn get_u8(&self) -> u8 {
        let v = *self.buf.as_slice()[self.read_index..self.read_index + mem::size_of::<u8>()].first().unwrap();
        v
    }

    pub fn get_i8(&self) -> i8 {
        let v = self.buf.as_slice()[self.read_index..self.read_index + mem::size_of::<i8>()].first().unwrap();
        *v as i8
    }

    fn get_u16_bytes(&self) -> [u8; mem::size_of::<u16>()] {
        let len = mem::size_of::<u16>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<u16>()];
        x.copy_from_slice(byte);
        x
    }

    pub fn get_u16_be(&self) -> u16 {
        u16::from_be_bytes(self.get_u16_bytes())
    }
    pub fn get_u16_le(&self) -> u16 {
        u16::from_le_bytes(self.get_u16_bytes())
    }


    fn get_i16_bytes(&self) -> [u8; mem::size_of::<i16>()] {
        let len = mem::size_of::<i16>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<i16>()];
        x.copy_from_slice(byte);
        x
    }

    pub fn get_i16_be(&self) -> i16 {
        i16::from_be_bytes(self.get_i16_bytes())
    }

    pub fn get_i16_le(&self) -> i16 {
        i16::from_le_bytes(self.get_i16_bytes())
    }


    fn get_u32_bytes(&self) -> [u8; mem::size_of::<u32>()] {
        let len = mem::size_of::<u32>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<u32>()];
        x.copy_from_slice(byte);
        x
    }
    pub fn get_u32_be(&self) -> u32 {
        u32::from_be_bytes(self.get_u32_bytes())
    }

    pub fn get_u32_le(&self) -> u32 {
        u32::from_le_bytes(self.get_u32_bytes())
    }


    fn get_i32_bytes(&self) -> [u8; mem::size_of::<i32>()] {
        let len = mem::size_of::<i32>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<i32>()];
        x.copy_from_slice(byte);
        x
    }

    pub fn get_i32_be(&self) -> i32 {
        i32::from_be_bytes(self.get_i32_bytes())
    }

    pub fn get_i32_le(&self) -> i32 {
        i32::from_le_bytes(self.get_i32_bytes())
    }

    fn get_u64_bytes(&self) -> [u8; mem::size_of::<u64>()] {
        let len = mem::size_of::<u64>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<u64>()];
        x.copy_from_slice(byte);
        x
    }

    pub fn get_u64_be(&self) -> u64 {
        u64::from_be_bytes(self.get_u64_bytes())
    }

    pub fn get_u64_le(&self) -> u64 {
        u64::from_le_bytes(self.get_u64_bytes())
    }


    fn get_i64_bytes(&self) -> [u8; mem::size_of::<i64>()] {
        let len = mem::size_of::<i64>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<i64>()];
        x.copy_from_slice(byte);
        x
    }

    pub fn get_i64_be(&self) -> i64 {
        i64::from_be_bytes(self.get_i64_bytes())
    }

    pub fn get_i64_le(&self) -> i64 {
        i64::from_le_bytes(self.get_i64_bytes())
    }


    fn get_u128_bytes(&self) -> [u8; mem::size_of::<u128>()] {
        let len = mem::size_of::<u128>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<u128>()];
        x.copy_from_slice(byte);
        x
    }

    pub fn get_u128_be(&self) -> u128 {
        u128::from_be_bytes(self.get_u128_bytes())
    }

    pub fn get_u128_le(&self) -> u128 {
        u128::from_le_bytes(self.get_u128_bytes())
    }

    fn get_i128_bytes(&self) -> [u8; mem::size_of::<i128>()] {
        let len = mem::size_of::<i128>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<i128>()];
        x.copy_from_slice(byte);
        x
    }

    pub fn get_i128_be(&self) -> i128 {
        i128::from_be_bytes(self.get_i128_bytes())
    }

    pub fn get_i128_le(&self) -> i128 {
        i128::from_le_bytes(self.get_i128_bytes())
    }

    fn get_f32_bytes(&self) -> [u8; mem::size_of::<f32>()] {
        let len = mem::size_of::<f32>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<f32>()];
        x.copy_from_slice(byte);
        x
    }

    pub fn get_f32_be(&self) -> f32 {
        f32::from_be_bytes(self.get_f32_bytes())
    }

    pub fn get_f32_le(&self) -> f32 {
        f32::from_le_bytes(self.get_f32_bytes())
    }

    fn get_f64_bytes(&self) -> [u8; mem::size_of::<f64>()] {
        let len = mem::size_of::<f64>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<f64>()];
        x.copy_from_slice(byte);
        x
    }

    pub fn get_f64_be(&self) -> f64 {
        f64::from_be_bytes(self.get_f64_bytes())
    }

    pub fn get_f64_le(&self) -> f64 {
        f64::from_be_bytes(self.get_f64_bytes())
    }


    pub fn read_string(&mut self, len: usize) -> String {
        if len == 0 {
            return String::from("");
        }
        let x = &self.buf.as_slice()[self.read_index..self.read_index + len];
        self.read_index += len;
        String::from_utf8_lossy(x).to_string()
    }


    pub fn read_string_with_u8_be_len(&mut self) -> String {
        let len = self.read_u8();
        self.read_string(len as usize)
    }

    pub fn read_string_with_u16_be_len(&mut self) -> String {
        let len = self.read_u16_be();
        self.read_string(len as usize)
    }

    pub fn read_string_with_u16_le_len(&mut self) -> String {
        let len = self.read_u16_le();
        self.read_string(len as usize)
    }


    pub fn read_string_with_u32_be_len(&mut self) -> String {
        let len = self.read_u32_be();
        self.read_string(len as usize)
    }

    pub fn read_string_with_u32_le_len(&mut self) -> String {
        let len = self.read_u32_le();
        self.read_string(len as usize)
    }


    pub fn read_bytes(&mut self, bytes: &mut [u8]) {
        let n = self.get_bytes(bytes);
        self.read_index += n;
    }

    pub fn read_bool(&mut self) -> bool {
        let r = self.get_bool();
        self.read_index += 1;
        r
    }

    pub fn read_u8(&mut self) -> u8 {
        let r = self.get_u8();
        self.read_index += 1;
        r
    }

    pub fn read_i8(&mut self) -> i8 {
        let r = self.get_i8();
        self.read_index += 1;
        r
    }

    pub fn read_u16_be(&mut self) -> u16 {
        let len = mem::size_of::<u16>();
        let r = self.get_u16_be();
        self.read_index += len;
        r
    }

    pub fn read_u16_le(&mut self) -> u16 {
        let len = mem::size_of::<u16>();
        let r = self.get_u16_le();
        self.read_index += len;
        r
    }

    pub fn read_i16_be(&mut self) -> i16 {
        let len = mem::size_of::<i16>();
        let r = self.get_i16_be();
        self.read_index += len;
        r
    }


    pub fn read_i16_le(&mut self) -> i16 {
        let len = mem::size_of::<i16>();
        let r = self.get_i16_le();
        self.read_index += len;
        r
    }


    pub fn read_u32_be(&mut self) -> u32 {
        let len = mem::size_of::<u32>();
        let r = self.get_u32_be();
        self.read_index += len;
        return r;
    }

    pub fn read_u32_le(&mut self) -> u32 {
        let len = mem::size_of::<u32>();
        let r = self.get_u32_le();
        self.read_index += len;
        r
    }


    pub fn read_i32_be(&mut self) -> i32 {
        let len = mem::size_of::<i32>();
        let r = self.get_i32_be();
        self.read_index += len;
        r
    }

    pub fn read_i32_le(&mut self) -> i32 {
        let len = mem::size_of::<i32>();
        let r = self.get_i32_le();
        self.read_index += len;
        r
    }

    pub fn read_u64_be(&mut self) -> u64 {
        let len = mem::size_of::<u64>();
        let r = self.get_u64_be();
        self.read_index += len;
        r
    }


    pub fn read_u64_le(&mut self) -> u64 {
        let len = mem::size_of::<u64>();
        let r = self.get_u64_le();
        self.read_index += len;
        r
    }


    pub fn read_i64_be(&mut self) -> i64 {
        let len = mem::size_of::<i64>();
        let r = self.get_i64_be();
        self.read_index += len;
        r
    }

    pub fn read_i64_le(&mut self) -> i64 {
        let len = mem::size_of::<i64>();
        let r = self.get_i64_le();
        self.read_index += len;
        r
    }

    pub fn read_u128_be(&mut self) -> u128 {
        let len = mem::size_of::<u128>();
        let r = self.get_u128_be();
        self.read_index += len;
        r
    }

    pub fn read_u128_le(&mut self) -> u128 {
        let len = mem::size_of::<u128>();
        let r = self.get_u128_le();
        self.read_index += len;
        r
    }

    pub fn read_i128_be(&mut self) -> i128 {
        let len = mem::size_of::<i128>();
        let r = self.get_i128_be();
        self.read_index += len;
        r
    }

    pub fn read_i128_le(&mut self) -> i128 {
        let len = mem::size_of::<i128>();
        let r = self.get_i128_le();
        self.read_index += len;
        r
    }


    pub fn read_f32_be(&mut self) -> f32 {
        let len = mem::size_of::<f32>();
        let r = self.get_f32_be();
        self.read_index += len;
        r
    }

    pub fn read_f32_le(&mut self) -> f32 {
        let len = mem::size_of::<f32>();
        let r = self.get_f32_le();
        self.read_index += len;
        r
    }

    pub fn read_f64_be(&mut self) -> f64 {
        let len = mem::size_of::<f64>();
        let r = self.get_f64_be();
        self.read_index += len;
        r
    }

    pub fn read_f64_le(&mut self) -> f64 {
        let len = mem::size_of::<f64>();
        let r = self.get_f64_le();
        self.read_index += len;
        r
    }


    pub fn write_str(&mut self, v: &str) -> io::Result<usize> {
        let result = self.buf.write(v.as_bytes());
        self.write_index += v.as_bytes().len();
        result
    }


    pub fn write_bytebuf(&mut self, bb: ByteBuf) -> io::Result<usize> {
        let result = self.buf.write(bb.as_slice());
        self.write_index += bb.as_slice().len();
        result
    }


    pub fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let result = self.buf.write(bytes);
        self.write_index += bytes.len();
        result
    }

    fn write_string(&mut self, v: String) -> io::Result<usize> {
        let result = self.write_bytes(v.as_bytes());
        result
    }


    // u8 bigendian same as smallendian

    pub fn write_string_with_u8_be_len(&mut self, v: String) -> ByteReult<usize> {
        if v.as_bytes().len() > u8::MAX as usize {
            return Err(ByteBufError::new("StringLength out of  u8 max_value ".to_string()));
        }
        // write length
        self.write_u8_be(v.as_bytes().len() as u8);
        // write string
        if let Ok(l) = self.write_string(v) {
            Ok(l)
        } else {
            return Err(ByteBufError::new("nothing to write".to_string()));
        }
    }


    pub fn write_string_with_u16_be_len(&mut self, v: String) -> ByteReult<usize> {
        if v.as_bytes().len() > u16::MAX as usize {
            return Err(ByteBufError::new("StringLength out of  u8 max_value ".to_string()));
        }
        // write length
        self.write_u16_be(v.as_bytes().len() as u16);
        // write string
        if let Ok(l) = self.write_string(v) {
            Ok(l)
        } else {
            return Err(ByteBufError::new("nothing to write".to_string()));
        }
    }

    pub fn write_string_with_u16_le_len(&mut self, v: String) -> ByteReult<usize> {
        if v.as_bytes().len() > u16::MAX as usize {
            return Err(ByteBufError::new("StringLength out of  u8 max_value ".to_string()));
        }
        // write length
        self.write_u16_le(v.as_bytes().len() as u16);
        // write string
        if let Ok(l) = self.write_string(v) {
            Ok(l)
        } else {
            return Err(ByteBufError::new("nothing to write".to_string()));
        }
    }


    pub fn write_string_with_u32_be_len(&mut self, v: String) -> ByteReult<usize> {
        if v.as_bytes().len() > u16::MAX as usize {
            return Err(ByteBufError::new("StringLength out of  u8 max_value ".to_string()));
        }
        // write length
        self.write_u32_be(v.as_bytes().len() as u32);
        // write string
        if let Ok(l) = self.write_string(v) {
            Ok(l)
        } else {
            return Err(ByteBufError::new("nothing to write".to_string()));
        }
    }

    pub fn write_string_with_u32_le_len(&mut self, v: String) -> ByteReult<usize> {
        if v.as_bytes().len() > u16::MAX as usize {
            return Err(ByteBufError::new("StringLength out of  u8 max_value ".to_string()));
        }
        // write length
        self.write_u32_le(v.as_bytes().len() as u32);
        // write string
        if let Ok(l) = self.write_string(v) {
            Ok(l)
        } else {
            return Err(ByteBufError::new("nothing to write".to_string()));
        }
    }


    pub fn write_bool(&mut self, v: bool) -> io::Result<usize> {
        let r = if v {
            self.buf.write(&[0x01])
        } else {
            self.buf.write(&[0x00])
        };
        self.write_index += 1;
        r
    }

    pub fn write_u8_be(&mut self, v: u8) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += 1;
        r
    }

    pub fn write_u8_le(&mut self, v: u8) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += 1;
        r
    }

    pub fn write_i8_be(&mut self, v: i8) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += 1;
        r
    }

    pub fn write_i8_le(&mut self, v: i8) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += 1;
        r
    }

    pub fn write_u16_be(&mut self, v: u16) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<u16>();
        r
    }

    pub fn write_u16_le(&mut self, v: u16) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<u16>();
        r
    }

    pub fn write_i16_be(&mut self, v: i16) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<i16>();
        r
    }

    pub fn write_i16_le(&mut self, v: i16) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<i16>();
        r
    }


    pub fn write_u32_be(&mut self, v: u32) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<u32>();
        r
    }

    pub fn write_u32_le(&mut self, v: u32) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<u32>();
        r
    }


    pub fn write_i32_be(&mut self, v: i32) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<i32>();
        r
    }

    pub fn write_i32_le(&mut self, v: i32) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<i32>();
        r
    }


    pub fn write_u64_be(&mut self, v: u64) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<u64>();
        r
    }

    pub fn write_u64_le(&mut self, v: u64) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<u64>();
        r
    }

    pub fn write_i64_be(&mut self, v: i64) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<i64>();
        r
    }
    pub fn write_i64_le(&mut self, v: i64) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<i64>();
        r
    }


    pub fn write_u128_be(&mut self, v: u128) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<u128>();

        r
    }

    pub fn write_u128_le(&mut self, v: u128) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<u128>();

        r
    }

    pub fn write_i128_be(&mut self, v: i128) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<i128>();
        r
    }

    pub fn write_i128_le(&mut self, v: i128) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<i128>();
        r
    }

    pub fn write_f32_be(&mut self, v: f32) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<f32>();
        r
    }

    pub fn write_f32_le(&mut self, v: f32) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<f32>();
        r
    }


    pub fn write_f64_be(&mut self, v: f64) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<f64>();
        r
    }

    pub fn write_f64_le(&mut self, v: f64) -> io::Result<usize> {
        let r = self.buf.write(&v.to_le_bytes());
        self.write_index += mem::size_of::<f64>();
        r
    }


    pub fn skip_index(&mut self, n: usize) -> Result<usize, ByteBufError> {
        if n > self.write_index {
            let result1 = Err(ByteBufError {
                message: "exceed writeIndex range ".to_string()
            });
            return result1;
        };
        self.read_index += n;
        Ok(self.read_index)
    }

    pub fn readable_bytes(&self) -> usize {
        self.get_writer_index() - self.get_reader_index()
    }


    pub fn available_bytes(&self) -> &[u8] {
        return &self.buf[..self.get_writer_index()];
    }

    pub fn is_readable(&self) -> bool {
        let r = if self.readable_bytes() > 0 {
            true
        } else {
            false
        };
        r
    }


    pub fn is_writable(&self) -> bool {
        let r = if (self.capacity - self.write_index) > 0 {
            true
        } else {
            false
        };
        r
    }

    pub fn get_writer_index(&self) -> usize {
        self.write_index
    }


    pub fn get_reader_index(&self) -> usize {
        self.read_index
    }

    pub fn mark_reader_index(&mut self) {
        self.read_mark = self.read_index as isize
    }

    pub fn set_reader_index(&mut self, v: usize) {
        self.read_index = v;
    }

    pub fn reset_reader_index(&mut self) {
        if self.read_mark != -1 {
            self.read_index = self.read_mark as usize;
            self.read_mark = -1;
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        return self.buf.as_slice();
    }


    pub fn print_bytes(&self) {
        let mut i = 1;
        self.buf.iter().for_each(|x| {
            print!("{}", x);
            if i % 20 == 0 {
                println!()
            }
            i = i + 1;
        });
        println!()
    }
}




