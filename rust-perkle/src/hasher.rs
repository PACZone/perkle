pub trait Hasher: Clone {
    type Hash: Copy + PartialEq + Into<Vec<u8>> + TryFrom<Vec<u8>>;

    fn hash(data: &[u8]) -> Self::Hash;
}
