pub trait FixedXor: AsRef<[u8]> + FromIterator<u8> {
    fn fixed_xor_with<T: FixedXor>(&self, other: &T) -> Result<Self, SizeMismatchError>;
}

#[derive(Debug)]
pub struct SizeMismatchError;

impl<T: AsRef<[u8]> + FromIterator<u8>> FixedXor for T {
    fn fixed_xor_with<O: FixedXor>(&self, other: &O) -> Result<Self, SizeMismatchError> {
        let self_bytes = self.as_ref();
        let other_bytes = other.as_ref();

        if self_bytes.len() != other_bytes.len() {
            return Err(SizeMismatchError);
        }

        Ok(self_bytes
            .iter()
            .zip(other_bytes)
            .map(|(b1, b2)| b1 ^ b2)
            .collect::<T>())
    }
}
