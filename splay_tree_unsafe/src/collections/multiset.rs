//! 多重集合

use std::{
    fmt::Debug,
    ops::{Bound, RangeBounds},
};

use crate::{
    node::{
        find::{lower_bound, upper_bound},
        insert::{insert, insert_right},
        iterator::{prev, NodeIterator, NodePosition, NodeRangeIterator},
        pointer::{NodeOps, NodePtr},
        remove::remove,
        splay::splay,
    },
    utils::print::print_as_tree,
};

/// Multiset
/// - 多重集合
pub struct Multiset<K: Ord> {
    pub root: Option<NodePtr<K, usize>>,
    size: usize,
}

impl<K: Ord> Multiset<K> {
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

    /// 値 `x` を持つノードのうち，最も右側にあるものを探索する
    fn find_rightmost_node(&mut self, key: &K) -> Option<NodePtr<K, usize>> {
        let upperbound = prev(
            {
                let ub;
                (self.root, ub) = upper_bound(self.root.clone(), &key);
                ub
            },
            &self.root,
        );

        match upperbound {
            NodePosition::Node(node) if &*node.key() == key => Some(node),
            _ => None,
        }
    }

    /// 要素の追加
    pub fn insert(&mut self, key: K) {
        // 最も右側の頂点を探索
        let rightmost = self.find_rightmost_node(&key);

        let new_node;
        if let Some(rightmost) = rightmost {
            let cnt = *rightmost.value();
            new_node = insert_right(Some(rightmost), key, cnt + 1);
        } else {
            (_, new_node, _) = insert(self.root.clone(), key, 1);
        }

        self.size += 1;
        self.root = Some(splay(new_node));
    }

    /// 要素の削除
    pub fn remove(&mut self, key: &K) -> bool {
        // 最も右側の頂点を探索
        let Some(rightmost) = self.find_rightmost_node(&key) else {
            return false;
        };

        (self.root, _) = remove(rightmost);

        self.size -= 1;
        true
    }

    /// `key` に一致する要素の個数を返す
    pub fn count(&mut self, key: &K) -> usize {
        // 最も右側の頂点を探索
        let rightmost = self.find_rightmost_node(&key);

        if let Some(rightmost) = rightmost {
            *rightmost.value()
        } else {
            0
        }
    }

    /// 指定した区間のイテレータを返す
    pub fn range<R: RangeBounds<K>>(&mut self, range: R) -> NodeRangeIterator<K, usize> {
        let left = match range.start_bound() {
            Bound::Unbounded => NodePosition::INF,
            Bound::Included(x) => prev(
                {
                    let lb;
                    (self.root, lb) = lower_bound(self.root.clone(), &x);
                    lb
                },
                &self.root,
            ),
            Bound::Excluded(x) => prev(
                {
                    let ub;
                    (self.root, ub) = upper_bound(self.root.clone(), &x);
                    ub
                },
                &self.root,
            ),
        };
        let right = match range.end_bound() {
            Bound::Unbounded => NodePosition::SUP,
            Bound::Included(x) => {
                let ub;
                (self.root, ub) = upper_bound(self.root.clone(), x);
                ub
            }
            Bound::Excluded(x) => {
                let lb;
                (self.root, lb) = lower_bound(self.root.clone(), x);
                lb
            }
        };

        NodeRangeIterator::new(&self.root, left, right)
    }

    /// ノードのイテレータを返す
    pub fn iter(&self) -> NodeIterator<K, usize> {
        NodeIterator::first(&self.root)
    }
}

impl<K: Ord + Clone + Debug> Debug for Multiset<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set()
            .entries(NodeIterator::first(&self.root).map(|node| node.key().clone()))
            .finish()
    }
}

impl<K: Ord + Clone + Debug> Multiset<K> {
    pub fn print_as_tree(&self) {
        print_as_tree(&self.root);
    }
}
