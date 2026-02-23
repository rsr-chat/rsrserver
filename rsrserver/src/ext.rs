pub trait StrExt {
    fn slice_at_most(&self, bytes: usize) -> &str;
}
impl StrExt for &str {
    fn slice_at_most(&self, bytes: usize) -> &str {
        let n = bytes.min(self.len());
        (0..=n)
            .rev()
            .find(|&i| self.is_char_boundary(i))
            .map(|i| &self[..i])
            .unwrap_or("")
    }
}
