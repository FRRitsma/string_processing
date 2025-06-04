pub trait ByteSliceable {
    fn as_bytes(&self) -> &[u8];
    fn len(&self) -> usize;
}

impl ByteSliceable for &str {
    fn as_bytes(&self) -> &[u8] {
        str::as_bytes(self)
    }

    fn len(&self) -> usize {
        str::len(self)
    }
}

impl ByteSliceable for String {
    fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: ByteSliceable> ByteSliceable for &T {
    fn as_bytes(&self) -> &[u8] {
        T::as_bytes(self) // Call the trait method directly
    }
    fn len(&self) -> usize {
        T::len(self)
    }
}
