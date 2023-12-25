use crate::{
    map::AATreeMap,
    node::{AATreeNode, AATreeNodeInner},
};
use std::collections::VecDeque;

// ----- iterator -----
pub struct AATreeIterator<'a, K: 'a + Ord, V: 'a> {
    unvisited: VecDeque<&'a AATreeNodeInner<K, V>>,
}

impl<'a, K: Ord, V> AATreeIterator<'a, K, V> {
    fn push_left_edge(&mut self, mut tree: &'a AATreeNode<K, V>) {
        while let Some(node) = tree.as_deref() {
            self.unvisited.push_front(node);
            tree = &node.left;
        }
    }
}

impl<'a, K: Ord, V> Iterator for AATreeIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(node) = self.unvisited.pop_front() else {
            return None;
        };

        self.push_left_edge(&node.right);

        Some((&node.key, &node.value))
    }
}

impl<K: Ord, V> AATreeMap<K, V> {
    pub fn iter<'a>(&'a self) -> AATreeIterator<'a, K, V> {
        let mut iter = AATreeIterator {
            unvisited: VecDeque::new(),
        };
        iter.push_left_edge(&self.root);
        iter
    }
}

impl<'a, K: Ord, V> IntoIterator for &'a AATreeMap<K, V> {
    type IntoIter = AATreeIterator<'a, K, V>;
    type Item = (&'a K, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
