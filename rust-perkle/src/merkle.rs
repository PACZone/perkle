use std::collections::HashMap;

use crate::Hasher;

struct Node<T: Hasher> {
    width: usize,
    height: usize,
    hasher: T::Hash,
}

pub struct Tree<T: Hasher> {
    nodes: HashMap<usize, Node<T>>,
    max_width: usize,
    max_height: usize,

    hasher: T,
}
