pub fn empty_to_none(s: Option<String>) -> Option<String> {
    match s {
        Some(s) => {
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }
        None => None,
    }
}
