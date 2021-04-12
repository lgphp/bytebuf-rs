#[cfg(test)]
pub mod test {
    use crate::bytebuf::ByteBuf;

    #[test]
    fn test_bytebuf() {
        let mut buf = ByteBuf::new_with_capacity(0);
        let s = "我是中国人".to_string();
        let len = s.as_bytes().len();
        buf.write_string(s);
        println!("get_reader_index : {}", buf.get_reader_index());
        println!("get_write_index : {}", buf.get_writer_index());
        let result = buf.read_string(len);
        println!("result:{}", result);

    }
}