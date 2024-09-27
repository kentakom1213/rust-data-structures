use super::{
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
pub fn prev<K: Ord, V>(iter: NodePosition<K, V>, root: &NodePtr<K, V>) -> NodePosition<K, V> {
    match iter {
        NodePosition::INF => NodePosition::INF,
        NodePosition::Node(mut node) => {
            if let Some(left) = node.left().map(|node| node.clone()) {
                if let Some(mut prv) = left {
                    while let Some(right) =
                        Some(prv.clone()).right().map(|node| node.clone()).unwrap()
                    {
                        prv = right;
                    }
                    return NodePosition::Node(Some(prv));
                }
            }

            // 親をたどる
            while node.is_child() {
                match node.get_state() {
                    NodeState::LeftChild => {
                        node = node.get_parent_ptr();
                    }
                    NodeState::RightChild => {
                        return NodePosition::Node(node.get_parent_ptr());
                    }
                    _ => unreachable!(),
                }
            }

            NodePosition::INF
        }
        NodePosition::SUP => NodePosition::Node(get_max(root.clone())),
    }
}

/// 次に大きい値をもつノードを返す
///
/// - 計算量： `O(1) amotized`
pub fn next<K: Ord, V>(iter: NodePosition<K, V>, root: &NodePtr<K, V>) -> NodePosition<K, V> {
    match iter {
        NodePosition::INF => NodePosition::Node(get_min(root.clone())),
        NodePosition::Node(mut node) => {
            if let Some(right) = node.right().map(|node| node.clone()) {
                if let Some(mut nxt) = right {
                    while let Some(left) =
                        Some(nxt.clone()).left().map(|node| node.clone()).unwrap()
                    {
                        nxt = left;
                    }
                    return NodePosition::Node(Some(nxt));
                }
            }

            // 親をたどる
            while node.is_child() {
                match node.get_state() {
                    NodeState::RightChild => {
                        node = node.get_parent_ptr();
                    }
                    NodeState::LeftChild => {
                        return NodePosition::Node(node.get_parent_ptr());
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
pub fn get_min<K: Ord, V>(root: NodePtr<K, V>) -> NodePtr<K, V> {
    let mut node = root;

    while let left @ Some(_) = node.left().map(|node| node.clone())? {
        node = left;
    }

    node
}

/// rootを根とする木のうち，最も右側の子を返す
pub fn get_max<K: Ord, V>(root: NodePtr<K, V>) -> NodePtr<K, V> {
    let mut node = root;

    while let right @ Some(_) = node.right().map(|node| node.clone())? {
        node = right;
    }

    node
}

/// ノードのイテレータ
pub struct NodeIterator<'a, K: Ord, V> {
    /// 根のポインタ
    root: &'a NodePtr<K, V>,
    /// 現在の位置
    pos: NodePosition<K, V>,
}

impl<'a, K: Ord, V> NodeIterator<'a, K, V> {
    /// 新しいイテレータを返す
    pub fn new(root: &'a NodePtr<K, V>, node: NodePtr<K, V>) -> Self {
        NodeIterator {
            root,
            pos: NodePosition::Node(node),
        }
    }

    /// 左端のイテレータを返す
    pub fn first(root: &'a NodePtr<K, V>) -> Self {
        NodeIterator {
            root,
            pos: next(NodePosition::INF, root),
        }
    }

    /// 右端のイテレータを返す
    pub fn last(root: &'a NodePtr<K, V>) -> Self {
        NodeIterator {
            root,
            pos: prev(NodePosition::SUP, root),
        }
    }
}

impl<'a, K: Ord, V> Iterator for NodeIterator<'a, K, V> {
    type Item = NodePtr<K, V>;
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.pos.as_ref().map(|node| node.clone())??;
        // posを次に進める
        self.pos = next(self.pos.clone(), self.root);

        Some(Some(val))
    }
}

impl<'a, K: Ord, V> DoubleEndedIterator for NodeIterator<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let val = self.pos.as_ref().map(|node| node.clone())??;
        // posを前に進める
        self.pos = prev(self.pos.clone(), self.root);

        Some(Some(val))
    }
}

#[cfg(test)]
mod test_prev_next {
    use crate::{
        node::{
            find::find,
            insert::insert,
            iterator::{get_min, next, prev, NodePosition},
            pointer::NodeOps,
        },
        print_util::print_as_tree,
    };

    use super::NodeIterator;

    #[test]
    fn test_min() {
        let mut root = None;
        let items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _, _) = insert(root, i, i);
        }

        print_as_tree(&root);

        let min = get_min(root.clone());

        assert_eq!(*min.key().unwrap(), -1);
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
            assert_eq!(*itr.as_ref().unwrap().key().unwrap(), *i);

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
            assert_eq!(*itr.as_ref().unwrap().key().unwrap(), i);

            itr = next(itr, &root);
        }

        assert!(itr.is_sup());
    }

    #[test]
    fn test_iter() {
        let mut root = None;

        assert!(NodeIterator::first(&root).next().is_none());

        for i in [2, 5, 3, 8, 6, 1, 4, 7, 9, 10] {
            (root, _, _) = insert(root, i, i);
        }

        print_as_tree(&root);

        let itr = NodeIterator::first(&root);

        for (x, ans) in itr.zip([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) {
            assert_eq!(*x.key().unwrap(), ans);
        }

        let itr = NodeIterator::last(&root);

        for (x, ans) in itr.rev().zip([10, 9, 8, 7, 6, 5, 4, 3, 2, 1]) {
            assert_eq!(*x.key().unwrap(), ans);
        }

        let itr = NodeIterator::new(&root, find(root.clone(), &4));

        for (x, ans) in itr.zip([4, 5, 6, 7, 8, 9, 10]) {
            assert_eq!(*x.key().unwrap(), ans);
        }
    }
}
