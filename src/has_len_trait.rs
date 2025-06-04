pub(crate) trait HasLen {
    fn len(&self) -> usize;
}

impl HasLen for String {
    fn len(&self) -> usize {
        std::string::String::len(self)
    }
}

impl HasLen for &str {
    fn len(&self) -> usize {
        <str>::len(self)
    }
}
