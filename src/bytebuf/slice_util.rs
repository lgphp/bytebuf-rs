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