#[cfg(test)]
pub mod test {
    use std::ops::{Deref, DerefMut};

    use crate::bytebuf::ByteBuf;

    #[test]
    fn test_bytebuf() {
        let mut buf = ByteBuf::new_with_capacity(0);
        println!("bytebuf capacity :{}", buf.capacity());

        let s = "hello world".to_string();

        buf.write_u32_be(40u32);
        buf.write_u64_be(18u64);
        buf.write_string_with_u8_be_len(s.clone());

        buf.write_bytes(s.as_bytes());

        buf.set_f64_be(4, 89.001f64);


        println!("buf redadindex :{}", buf.get_reader_index());

        let i = buf.read_u32_be();
        let b = buf.read_f64_be();
        let rs = buf.read_string_with_u8_be_len();
        let mut bytes = &mut vec![0u8; s.as_bytes().len()][..];
        buf.read_bytes(bytes);

        buf.print_bytes();
        println!("read_u32 :{}", i);
        println!("read_u16 :{}", b);


        println!("read_string_with_u8_len :{}", rs);

        println!("bytes :{}", String::from_utf8_lossy(bytes));

        println!("available_bytes :{}", buf.available_bytes().len());
        println!("readable_bytes :{}", buf.readable_bytes());
        println!("as_slice :{}", buf.as_slice().len());


        println!("buf redadindex :{}", buf.get_reader_index());
        println!("buf writeIndex :{}", buf.get_writer_index());


        let buf_clone = buf.clone();
        println!("buf_clone redadindex :{}", buf_clone.get_reader_index());

        let buf_copy = buf.deep_clone();
        println!("buf_copy redadindex :{}", buf_copy.get_reader_index());
    }
}