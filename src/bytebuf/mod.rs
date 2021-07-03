#![allow(unused_imports , unused_variables)]
use std::error::Error;
use std::io::Write;
use core::mem;
use std::result::Result;
use std::{fmt, io};
use std::fmt::{Formatter, Debug};
use crate::error::ByteBufError;


const CHUNK_SIZE: usize = 1024;

pub struct ByteBuf {
    buf: Vec<u8>,
    capacity: usize,
    read_mark: isize,
    write_mark: isize,
    read_index: usize,
    write_index: usize,
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
            write_index: src.len() - 1,
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
            read_index: 0,
            write_index: *write_index,
        }
    }


    pub fn write_bytebuf(&mut self, bb: ByteBuf) -> io::Result<usize> {
        let result = self.buf.write(bb.as_slice());
        if self.write_index == 0 {
            self.write_index += bb.as_slice().len()-1;
        }else{
            self.write_index += bb.as_slice().len();
        }
        result
    }


    pub fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let result = self.buf.write(bytes);
        if self.write_index == 0 {
            self.write_index += bytes.len()-1;
        }else{
            self.write_index += bytes.len();
        }
        result
    }


    /// l : 要读取的字节长度
    pub fn read_bytes(&mut self, l: usize) -> &[u8] {
        let x = &self.buf.as_slice()[self.read_index..self.read_index + l];
        self.read_index += l;
        x
    }

    pub fn write_str(&mut self, v: &str) -> io::Result<usize> {
        let result = self.buf.write(v.as_bytes());
        if self.write_index==0{
            self.write_index += v.as_bytes().len()-1;
        }else{
            self.write_index += v.as_bytes().len();
        }
        result
    }

    pub fn write_string(&mut self, v: String) -> io::Result<usize> {
        let result = self.buf.write(v.as_bytes());
        if self.write_index==0{
            self.write_index += v.as_bytes().len()-1;
        }else{
            self.write_index += v.as_bytes().len();
        }
        result
    }

    pub fn read_string(&mut self, l: usize) -> String {
        let x = &self.buf.as_slice()[self.read_index..self.read_index + l];
        self.read_index += l;
        String::from_utf8_lossy(x).to_string()
    }


    pub fn write_bool(&mut self, v: bool) -> io::Result<usize> {
        let r = if v {
            self.buf.write(&[0x01])
        } else {
            self.buf.write(&[0x00])
        };
        if self.write_index>0{
            self.write_index += 1;
        }
        r
    }

    pub fn read_bool(&mut self) -> bool {
        let v = self.buf.as_slice()[self.read_index..self.read_index + mem::size_of::<bool>()].first().unwrap();
        self.read_index += 1;
        if *v == 0x01u8 {
            true
        } else {
            false
        }
    }


    pub fn write_u8(&mut self, v: u8) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index>0{
            self.write_index += 1;
        }
        r
    }

    pub fn read_u8(&mut self) -> u8 {
        let v = self.buf.as_slice()[self.read_index..self.read_index + mem::size_of::<u8>()].first().unwrap();
        self.read_index += 1;
        *v
    }


    pub fn write_i8(&mut self, v: i8) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index>0{
            self.write_index += 1;
        }
        r
    }

    pub fn read_i8(&mut self) -> i8 {
        let v = self.buf.as_slice()[self.read_index..self.read_index + mem::size_of::<i8>()].first().unwrap();
        self.read_index += 1;
        *v as i8
    }


    pub fn write_u16(&mut self, v: u16) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        self.write_index += mem::size_of::<u16>();
        r
    }

    pub fn read_u16(&mut self) -> u16 {
        let len = mem::size_of::<u16>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<u16>()];
        x.copy_from_slice(byte);
        let n = u16::from_be_bytes(x);
        self.read_index += len;
        n
    }


    pub fn write_i16(&mut self, v: i16) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index==0{
            self.write_index += mem::size_of::<i16>()-1;
        }else{
            self.write_index += mem::size_of::<i16>();
        }

        r
    }

    pub fn read_i16(&mut self) -> i16 {
        let len = mem::size_of::<i16>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<i16>()];
        x.copy_from_slice(byte);
        let n = i16::from_be_bytes(x);
        self.read_index += len;
        n
    }


    pub fn write_u32(&mut self, v: u32) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index==0{
            self.write_index += mem::size_of::<u32>()-1;
        }else{
            self.write_index += mem::size_of::<u32>();
        }
        r
    }

    pub fn read_u32(&mut self) -> u32 {
        let len = mem::size_of::<u32>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<u32>()];
        x.copy_from_slice(byte);
        let n = u32::from_be_bytes(x);
        self.read_index += len;
        n
    }

    pub fn write_i32(&mut self, v: i32) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index==0{
            self.write_index += mem::size_of::<i32>()-1;
        }else{
            self.write_index += mem::size_of::<i32>();
        }

        r
    }

    pub fn read_i32(&mut self) -> i32 {
        let len = mem::size_of::<i32>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<i32>()];
        x.copy_from_slice(byte);
        let n = i32::from_be_bytes(x);
        self.read_index += len;
        n
    }


    pub fn write_u64(&mut self, v: u64) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index==0{
            self.write_index += mem::size_of::<u64>()-1;
        }else{
            self.write_index += mem::size_of::<u64>();
        }
        r
    }

    pub fn read_u64(&mut self) -> u64 {
        let len = mem::size_of::<u64>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<u64>()];
        x.copy_from_slice(byte);
        let n = u64::from_be_bytes(x);
        self.read_index += len;
        n
    }


    pub fn write_i64(&mut self, v: i64) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index==0{
            self.write_index += mem::size_of::<i64>()-1;
        }else{
            self.write_index += mem::size_of::<i64>();
        }
        r
    }

    pub fn read_i64(&mut self) -> i64 {
        let len = mem::size_of::<i64>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<i64>()];
        x.copy_from_slice(byte);
        let n = i64::from_be_bytes(x);
        self.read_index += len;
        n
    }


    pub fn write_u128(&mut self, v: u128) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index==0{
            self.write_index += mem::size_of::<u128>()-1;
        }else{
            self.write_index += mem::size_of::<u128>();
        }
        r
    }


    pub fn read_u128(&mut self) -> u128 {
        let len = mem::size_of::<u128>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<u128>()];
        x.copy_from_slice(byte);
        let n = u128::from_be_bytes(x);
        self.read_index += len;
        n
    }

    pub fn write_i128(&mut self, v: i128) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index==0{
            self.write_index += mem::size_of::<i128>()-1;
        }else{
            self.write_index += mem::size_of::<i128>();
        }
        r
    }


    pub fn read_i128(&mut self) -> i128 {
        let len = mem::size_of::<i128>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<i128>()];
        x.copy_from_slice(byte);
        let n = i128::from_be_bytes(x);
        self.read_index += len;
        n
    }


    pub fn write_f32(&mut self, v: f32) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index==0{
            self.write_index += mem::size_of::<f32>()-1;
        }else{
            self.write_index += mem::size_of::<f32>();
        }
        r
    }
    pub fn read_f32(&mut self) -> f32 {
        let len = mem::size_of::<f32>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<f32>()];
        x.copy_from_slice(byte);
        let n = f32::from_be_bytes(x);
        self.read_index += len;
        n
    }


    pub fn write_f64(&mut self, v: f64) -> io::Result<usize> {
        let r = self.buf.write(&v.to_be_bytes());
        if self.write_index==0{
            self.write_index += mem::size_of::<f64>()-1;
        }else{
            self.write_index += mem::size_of::<f64>();
        }
        r
    }
    pub fn read_f64(&mut self) -> f64 {
        let len = mem::size_of::<f64>();
        let byte = &self.buf.as_slice()[self.read_index..self.read_index + len];
        let mut x = [0u8; mem::size_of::<f64>()];
        x.copy_from_slice(byte);
        let n = f64::from_be_bytes(x);
        self.read_index += len;
        n
    }

    pub fn skip_read_index(&mut self, n: usize) -> Result<usize, ByteBufError> {
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
        self.write_index - self.read_index
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


    /// get buf's write index
    ///  # Examples
    ///  ```
    ///   let mut buf = ByteBuf::new_with_capacity(0);
    ///         let s = "我是中国人".to_string();
    ///         let len = s.as_bytes().len();
    ///         buf.write_string(s);
    ///
    ///        println!("get_write_index : {}", buf.get_write_index());
    ///
    /// ```
    ///

    pub fn get_writer_index(&self) -> usize {
         self.write_index
    }


    pub fn get_reader_index(&self) -> usize {
        let r =  if  self.read_index > 0 {
            self.read_index - 1
        }else {
            self.read_index
        };
        r
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
}




