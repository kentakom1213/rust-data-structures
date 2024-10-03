use std::{cmp, fmt::Debug};

use crate::node::{
    pointer::{NodeOps, NodePtr},
    state::NodeState,
};

/// ノードの位置
#[derive(Debug)]
pub enum NodePosition<K: Ord, V> {
    /// `K` の下界
    INF,
    /// ノードの値
    Node(NodePtr<K, V>),
    /// `K` の上界
    SUP,
}

impl<K: Ord, V> Clone for NodePosition<K, V> {
    fn clone(&self) -> Self {
        match self {
            NodePosition::INF => NodePosition::INF,
            NodePosition::Node(node) => NodePosition::Node(node.clone()),
            NodePosition::SUP => NodePosition::SUP,
        }
    }
}

impl<K: Ord, V> PartialEq for NodePosition<K, V> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodePosition::INF, NodePosition::INF) => true,
            (NodePosition::SUP, NodePosition::SUP) => true,
            (NodePosition::Node(node1), NodePosition::Node(node2)) => node1.is_same(node2),
            _ => false,
        }
    }
}

impl<K: Ord, V> PartialOrd for NodePosition<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (NodePosition::INF, NodePosition::INF) => Some(cmp::Ordering::Equal),
            (NodePosition::SUP, NodePosition::SUP) => Some(cmp::Ordering::Equal),
            (NodePosition::Node(node1), NodePosition::Node(node2)) => Some(node1.key_cmp(node2)),
            (NodePosition::INF, _) => Some(cmp::Ordering::Less),
            (NodePosition::SUP, _) => Some(cmp::Ordering::Greater),
            (_, NodePosition::INF) => Some(cmp::Ordering::Greater),
            (_, NodePosition::SUP) => Some(cmp::Ordering::Less),
        }
    }
}

impl<K: Ord, V> NodePosition<K, V> {
    pub fn is_inf(&self) -> bool {
        match self {
            NodePosition::INF => true,
            _ => false,
        }
    }

    pub fn is_sup(&self) -> bool {
        match self {
            NodePosition::SUP => true,
            _ => false,
        }
    }

    pub fn is_node(&self) -> bool {
        match self {
            NodePosition::Node(_) => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            NodePosition::INF | NodePosition::SUP => true,
            _ => false,
        }
    }

    pub fn unwrap(self) -> NodePtr<K, V> {
        match self {
            NodePosition::Node(node) => node,
            _ => panic!("NodePosition::unwrap"),
        }
    }

    pub fn as_ref(&self) -> Option<&NodePtr<K, V>> {
        match self {
            NodePosition::Node(node) => Some(node),
            _ => None,
        }
    }
}

/// 次に小さい値を持つノードを返す
///
/// - 計算量： `O(1) amotized`
pub fn prev<K: Ord, V>(
    iter: NodePosition<K, V>,
    root: &Option<NodePtr<K, V>>,
) -> NodePosition<K, V> {
    match iter {
        NodePosition::INF => NodePosition::INF,
        NodePosition::Node(mut node) => {
            if let Some(mut prv) = node.left().as_ref().map(|node| node.clone()) {
                while let Some(right) = prv.clone().right().as_ref().map(|node| node.clone()) {
                    prv = right;
                }
                return NodePosition::Node(prv);
            }

            // 親をたどる
            while node.is_child() {
                match node.get_state() {
                    NodeState::LeftChild => {
                        // 親は存在する
                        node = node.parent().unwrap();
                    }
                    NodeState::RightChild => {
                        return NodePosition::Node(node.parent().unwrap());
                    }
                    _ => unreachable!(),
                }
            }

            NodePosition::INF
        }
        NodePosition::SUP => match get_max(root.clone()) {
            Some(node) => NodePosition::Node(node),
            None => NodePosition::SUP,
        },
    }
}

/// 次に大きい値をもつノードを返す
///
/// - 計算量： `O(1) amotized`
pub fn next<K: Ord, V>(
    iter: NodePosition<K, V>,
    root: &Option<NodePtr<K, V>>,
) -> NodePosition<K, V> {
    match iter {
        NodePosition::INF => match get_min(root.clone()) {
            Some(node) => NodePosition::Node(node),
            None => NodePosition::INF,
        },
        NodePosition::Node(mut node) => {
            if let Some(mut nxt) = node.right().as_ref().map(|node| node.clone()) {
                while let Some(left) = nxt.clone().left().as_ref().map(|node| node.clone()) {
                    nxt = left;
                }
                return NodePosition::Node(nxt);
            }

            // 親をたどる
            while node.is_child() {
                match node.get_state() {
                    NodeState::RightChild => {
                        // 親は存在する
                        node = node.parent().unwrap();
                    }
                    NodeState::LeftChild => {
                        return NodePosition::Node(node.parent().unwrap());
                    }
                    _ => unreachable!(),
                }
            }

            NodePosition::SUP
        }
        NodePosition::SUP => NodePosition::SUP,
    }
}

/// rootを根とする木のうち，最も左側の子を返す
pub fn get_min<K: Ord, V>(root: Option<NodePtr<K, V>>) -> Option<NodePtr<K, V>> {
    let mut node = root;

    while let left @ Some(_) = node.as_ref().map(|node| node.left().clone())? {
        node = left;
    }

    node
}

/// rootを根とする木のうち，最も右側の子を返す
pub fn get_max<K: Ord, V>(root: Option<NodePtr<K, V>>) -> Option<NodePtr<K, V>> {
    let mut node = root;

    while let right @ Some(_) = node.as_ref().map(|node| node.right().clone())? {
        node = right;
    }

    node
}

/// ノードのイテレータ
pub struct NodeIterator<'a, K: Ord, V> {
    /// 根のポインタ
    root: &'a Option<NodePtr<K, V>>,
    /// 現在の位置
    pos: NodePosition<K, V>,
}

impl<'a, K: Ord, V> NodeIterator<'a, K, V> {
    /// 新しいイテレータを返す
    pub fn new(root: &'a Option<NodePtr<K, V>>, node: NodePtr<K, V>) -> Self {
        NodeIterator {
            root,
            pos: NodePosition::Node(node),
        }
    }

    /// 左端のイテレータを返す
    pub fn first(root: &'a Option<NodePtr<K, V>>) -> Self {
        NodeIterator {
            root,
            pos: NodePosition::INF,
        }
    }

    /// 右端のイテレータを返す
    pub fn last(root: &'a Option<NodePtr<K, V>>) -> Self {
        NodeIterator {
            root,
            pos: NodePosition::SUP,
        }
    }
}

impl<'a, K: Ord, V> Iterator for NodeIterator<'a, K, V> {
    type Item = NodePtr<K, V>;
    fn next(&mut self) -> Option<Self::Item> {
        // posを次に進める
        self.pos = next(self.pos.clone(), self.root);

        let val = self.pos.as_ref().map(|node| node.clone())?;

        Some(val)
    }
}

impl<'a, K: Ord, V> DoubleEndedIterator for NodeIterator<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // posを前に進める
        self.pos = prev(self.pos.clone(), self.root);

        let val = self.pos.as_ref().map(|node| node.clone())?;

        Some(val)
    }
}

/// 範囲をもつイテレータ
pub struct NodeRangeIterator<'a, K: Ord, V> {
    /// 根のポインタ
    root: &'a Option<NodePtr<K, V>>,
    /// 左端の位置
    left: NodePosition<K, V>,
    /// 右端の位置
    right: NodePosition<K, V>,
}

impl<'a, K: Ord, V> NodeRangeIterator<'a, K, V> {
    /// 新しいイテレータを返す
    pub fn new(
        root: &'a Option<NodePtr<K, V>>,
        left: NodePosition<K, V>,
        right: NodePosition<K, V>,
    ) -> Self {
        NodeRangeIterator { root, left, right }
    }
}

impl<'a, K: Ord, V> Iterator for NodeRangeIterator<'a, K, V> {
    type Item = NodePtr<K, V>;
    fn next(&mut self) -> Option<Self::Item> {
        // 左端を次に進める
        self.left = next(self.left.clone(), self.root);

        // 左端が右端に到達したら終了
        if self.left >= self.right {
            return None;
        }

        let val = self.left.as_ref().map(|node| node.clone())?;

        Some(val)
    }
}

impl<'a, K: Ord + Debug, V: Debug> DoubleEndedIterator for NodeRangeIterator<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // 右端を前に進める
        self.right = prev(self.right.clone(), self.root);

        // 右端が左端に到達したら終了
        if self.right <= self.left {
            return None;
        }

        let val = self.right.as_ref().map(|node| node.clone())?;

        Some(val)
    }
}

#[cfg(test)]
mod test_prev_next {
    use crate::{
        node::{
            insert::insert,
            iterator::{get_min, next, prev, NodePosition},
            pointer::NodeOps,
            splay::splay,
        },
        utils::print::print_as_tree,
    };

    use super::{NodeIterator, NodeRangeIterator};

    #[test]
    fn test_min() {
        let mut root = None;
        let items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _, _) = insert(root, i, i);
        }

        print_as_tree(&root);

        let min = get_min(root.clone());

        assert_eq!(*min.unwrap().key(), -1);
    }

    #[test]
    fn test_prev() {
        let mut root = None;
        let mut items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _, _) = insert(root, i, i);
        }

        print_as_tree(&root);

        let mut itr = prev(NodePosition::SUP, &root);
        println!("itr: {:?}", itr);

        // アイテムをソート
        items.sort();

        for i in items.iter().rev() {
            assert_eq!(*itr.as_ref().unwrap().key(), *i);

            itr = prev(itr, &root);
            println!("itr: {:?}", itr);
        }

        assert!(itr.is_inf());
    }

    #[test]
    fn test_next() {
        let mut root = None;
        let mut items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _, _) = insert(root, i, i);
        }

        print_as_tree(&root);

        let mut itr = next(NodePosition::INF, &root);

        // アイテムをソート
        items.sort();

        for i in items {
            assert_eq!(*itr.as_ref().unwrap().key(), i);

            itr = next(itr, &root);
        }

        assert!(itr.is_sup());
    }

    #[test]
    fn test_iter() {
        let mut root = None;

        assert!(NodeIterator::first(&root).next().is_none());

        let mut node_4 = None;
        for i in [2, 5, 3, 8, 6, 1, 4, 7, 9, 10] {
            let inserted;
            (root, inserted, _) = insert(root, i, i);
            if i == 4 {
                node_4 = Some(inserted);
            }
        }

        print_as_tree(&root);

        let itr = NodeIterator::first(&root);

        for (x, ans) in itr.zip([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) {
            assert_eq!(*x.key(), ans);
        }

        let itr = NodeIterator::last(&root);

        for (x, ans) in itr.rev().zip([10, 9, 8, 7, 6, 5, 4, 3, 2, 1]) {
            assert_eq!(*x.key(), ans);
        }

        let mut itr = NodeIterator::new(&root, node_4.unwrap());
        itr.next_back();

        for (x, ans) in itr.zip([4, 5, 6, 7, 8, 9, 10]) {
            assert_eq!(*x.key(), ans);
        }
    }

    #[test]
    fn test_range_iterator() {
        let mut root = None;

        let data = vec![69, 39, 34, 21, 33, 52, 1000, -200];

        for &i in data.iter() {
            let inserted;
            (_, inserted, _) = insert(root.clone(), i, i);
            root = Some(splay(inserted));
        }

        print_as_tree(&root);

        let mut itr = NodeRangeIterator::new(&root, NodePosition::INF, NodePosition::SUP);

        assert_eq!(*itr.next().unwrap().key(), -200);
        assert_eq!(*itr.next().unwrap().key(), 21);
        assert_eq!(*itr.next_back().unwrap().key(), 1000);
        assert_eq!(*itr.next_back().unwrap().key(), 69);
    }
}
