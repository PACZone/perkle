use std::collections::HashMap;

use crate::Hasher;

#[derive(Clone, Debug, Copy)]
struct Node<T: Hasher> {
    width: usize,
    height: usize,
    hashe: Option<T::Hash>,
}

pub struct Tree<T: Hasher> {
    nodes: HashMap<usize, Node<T>>,
    max_width: usize,
    max_height: usize,

    hasher: T,
}

fn node_id(width: usize, height: usize) -> usize {
    ((height & 0xff) << 24) | (width & 0xffffff)
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

    fn get_node(&self, node_id: usize) -> Option<&Node<T>> {
        self.nodes.get(&node_id)
    }

    fn get_or_create_node(&mut self, width: usize, height: usize) -> Option<&Node<T>> {
        let node_id = node_id(width, height);
        let node = self.get_node(node_id);
        if let Some(n) = node {
            return Some(n);
        }

        let new_node = Node {
            width,
            height,
            hashe: None,
        };

        // self.nodes.insert(node_id, new_node).as_ref() //! Compiler error.
    }
}
