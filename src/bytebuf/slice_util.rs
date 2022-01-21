use std::fmt::Debug;
use std::ptr;

pub fn append<T: Clone>(src: &[T], dest: &[T]) -> Vec<T>
{
    let mut f = Vec::<T>::new();
    f.append(&mut src.to_vec());
    f.append(&mut dest.to_vec());
    f
}

pub fn insert_slice<T>(src: &[T], src_start: usize, src_end: usize, dest: &[T], dest_start: usize) -> Vec<T>
    where T: Sized + Clone {
    append(append(&dest[..dest_start], &src[src_start..src_end]).as_slice(), &dest[dest_start..])
}

pub fn delete_slice<T>(src: &[T], start: usize, len: usize) -> Vec<T>
    where T: Sized + Clone {
    append(&src[..start], &src[start + len..])
}

pub fn copy_slice<T>(src: &[T], src_start: usize, src_end: usize, dest: &[T], dest_start: usize) -> Vec<T>
    where T: Sized + Clone {
    append(&src[src_start..src_end], &dest[dest_start..])
}

pub trait ArrayCopy<T> where T: Sized {
    fn copy_to(&mut self, dest: &mut [T], dest_start: usize);
}

impl<T> ArrayCopy<T> for &mut [T] {
    fn copy_to(&mut self, dest: &mut [T], dest_start: usize) {
        unsafe {
            let s_ptr = self.as_mut_ptr();
            let src_ptr = s_ptr.add(0);
            let d_ptr = dest.as_mut_ptr();
            let dest_ptr = d_ptr.add(dest_start);
            ptr::copy(src_ptr, dest_ptr, self.len());
        }
    }
}

mod test {
    use crate::bytebuf::slice_util::ArrayCopy;

    #[test]
    pub fn test_slice_copy() {
        let mut s = "hello".to_string();
        let mut byte_a = unsafe { s.as_bytes_mut() };
        let mut byte_b = &mut vec![0u8; byte_a.len()][..];
        byte_a.copy_to(&mut byte_b, 1);
        println!("{}", String::from_utf8_lossy(byte_b));
    }
}