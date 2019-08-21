pub fn empty_to_none<T: AsRef<[u8]>>(s: Option<T>) -> Option<T> {
    match s {
        Some(s) => {
            if s.as_ref().is_empty() {
                None
            } else {
                Some(s)
            }
        }
        None => None,
    }
}
