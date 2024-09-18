//! Splay木による集合

use std::fmt::Debug;

use crate::node::{insert_multi, upper_bound, NodePtr};

/// Splay木による集合
pub struct MultiSet<K: Ord> {
    /// ルートノード
    root: NodePtr<K, ()>,
    /// 要素数
    size: usize,
}

impl<K: Ord> MultiSet<K> {
    /// 値`key`を追加する
    pub fn insert(&mut self, key: K) {
        let mut root = self.root.take();

        let new_node;
        (root, new_node) = insert_multi(root, key, ());

        if let Some(root) = root {
            self.root.replace(root);
        }
    }
}

impl<K: Ord> Default for MultiSet<K> {
    fn default() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }
}

impl<K: Ord + Debug> Debug for MultiSet<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
