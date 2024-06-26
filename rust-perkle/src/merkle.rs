use std::collections::HashMap;

use crate::Hasher;

#[derive(Clone, Debug, Copy)]
struct Node<T: Hasher> {
    width: usize,
    height: usize,
    hash: Option<T::Hash>,
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

    fn get_or_create_node(&mut self, width: usize, height: usize) -> Node<T> {
        let node_id = node_id(width, height);

        self.nodes
            .entry(node_id)
            .or_insert(Node {
                width,
                height,
                hash: None,
            })
            .clone()
    }

    fn invalidate_node(&mut self, width: usize, height: usize) {
        let mut node = self.get_or_create_node(width, height);
        node.hash = None;
    }

    fn recalculate_height(&mut self, max_width: usize) {
        if max_width > self.max_width {
            self.max_width = max_width;

            let max_height = (max_width as f64).log2(); // should we ude `f32` here?

            if max_height.rem_euclid(1.0) != 0.0 {
                self.max_height = (max_height.trunc() + 2.0) as usize;
            } else {
                self.max_height = (max_height.trunc() + 1.0) as usize;
            }
        }
    }

    // WIP!
    // fn node_hash(&mut self, width: usize, height: usize) -> Option<T::Hash> {
    //     let n_id = node_id(width, height);

    //     let node = self.get_node(n_id);
    //     if node.is_none() {
    //         let n_id = node_id(width, height);
    //         let node = self.get_node(n_id);
    //         if node.is_none() {
    //             return None;
    //         }
    //     }
    // }
}
