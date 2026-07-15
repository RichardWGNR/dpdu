pub fn is_valid_ptr<T>(ptr: *const T) -> bool {
    !ptr.is_null() && ptr.is_aligned()
}