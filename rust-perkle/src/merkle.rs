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

impl<T: Hasher> Node<T> {
    fn node_id(&self) -> usize {
        ((self.height & 0xff) << 24) | (self.width & 0xffffff)
    }
}

impl<T: Hasher> Tree<T> {
    pub fn new(hasher: T) -> Self {
        Self {
            nodes: HashMap::new(),
            max_width: 0,
            max_height: 0,
            hasher,
        }
    }
}
