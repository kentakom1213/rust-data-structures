//! 多重集合

use std::fmt::Debug;

use crate::{
    node::{
        find::upper_bound,
        insert::{insert, insert_right},
        iterator::{prev, NodeIterator, NodePosition},
        pointer::{NodeOps, NodePtr},
        splay::splay,
    },
    print_util::print_as_tree,
};

/// Multiset
/// - 多重集合
pub struct Multiset<K: Ord> {
    pub root: NodePtr<K, usize>,
    size: usize,
}

impl<K: Ord + Debug> Multiset<K> {
    /// 新規作成
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// 要素数
    pub fn len(&self) -> usize {
        self.size
    }

    /// 空判定
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 要素の追加
    pub fn insert(&mut self, key: K) {
        // keyをもつ最も右の頂点を探索
        let rightmost = prev(
            if let ub @ Some(_) = upper_bound(&self.root, &key) {
                NodePosition::Node(ub)
            } else {
                NodePosition::SUP
            },
            &self.root,
        );

        let new_node;
        match rightmost {
            NodePosition::Node(node) if node.key().is_some_and(|k| *k == key) => {
                let cnt = *node.value().unwrap();
                new_node = insert_right(node, key, cnt + 1);
            }
            _ => {
                (_, new_node, _) = insert(self.root.clone(), key, 1);
            }
        }

        eprintln!("Before Splay:");
        print_as_tree(&self.root);

        if let Some(right) = new_node.right() {
            println!("before right_state: {:?}", right.get_state());
        }

        self.size += 1;
        self.root = splay(new_node);

        eprintln!("After Splay:");
        print_as_tree(&self.root);

        if let Some(right) = self.root.right() {
            println!("after right_state: {:?}", right.get_state());
        }
    }
}

impl<K: Ord + Clone + Debug> Debug for Multiset<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set()
            .entries(NodeIterator::first(&self.root).map(|node| node.key().unwrap().clone()))
            .finish()
    }
}

impl<K: Ord + Clone + Debug> Multiset<K> {
    pub fn print_as_tree(&self) {
        print_as_tree(&self.root);
    }
}
