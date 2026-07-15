pub fn is_valid_ptr<T>(ptr: *const T) -> bool {
    !ptr.is_null() && ptr.is_aligned()
}

pub fn prepare_vector<T>(vec: &mut Vec<T>) {
    if vec.len() != vec.capacity() {
        vec.shrink_to_fit();
    }
}