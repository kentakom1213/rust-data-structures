use super::{pointer::NodeOps, state::NodeState, NodePtr};

/// ノードのイテレータ
#[derive(Debug)]
pub enum NodeIterator<'a, K: Ord, V> {
    /// `K` の下界
    INF(&'a NodePtr<K, V>),
    /// ノードの値
    Node {
        root: &'a NodePtr<K, V>,
        node: NodePtr<K, V>,
    },
    /// `K` の上界
    SUP(&'a NodePtr<K, V>),
}

impl<'a, K: Ord, V> NodeIterator<'a, K, V> {
    pub fn is_inf(&self) -> bool {
        match self {
            NodeIterator::INF(_) => true,
            _ => false,
        }
    }

    pub fn is_sup(&self) -> bool {
        match self {
            NodeIterator::SUP(_) => true,
            _ => false,
        }
    }

    pub fn is_node(&self) -> bool {
        match self {
            NodeIterator::Node { .. } => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            NodeIterator::INF(_) | NodeIterator::SUP(_) => true,
            _ => false,
        }
    }

    pub fn unwrap(self) -> NodePtr<K, V> {
        match self {
            NodeIterator::Node { node, .. } => node,
            _ => panic!("NodeIterator::unwrap"),
        }
    }

    pub fn as_ref(&'a self) -> Option<&'a NodePtr<K, V>> {
        match self {
            NodeIterator::Node { node, .. } => Some(node),
            _ => None,
        }
    }
}

/// 次に小さい値を持つノードを返す
///
/// - 計算量： `O(1) amotized`
pub fn prev<K: Ord, V>(iter: NodeIterator<K, V>, root: NodePtr<K, V>) -> NodeIterator<K, V> {
    match iter {
        NodeIterator::INF(root) => NodeIterator::INF(root),
        NodeIterator::Node { root, mut node } => {
            if let Some(left) = node.left().map(|node| node.clone()) {
                if let Some(mut prv) = left {
                    while let Some(right) =
                        Some(prv.clone()).right().map(|node| node.clone()).unwrap()
                    {
                        prv = right;
                    }
                    return NodeIterator::Node {
                        root,
                        node: Some(prv),
                    };
                }
            }

            // 親をたどる
            while node.is_child() {
                match node.get_state() {
                    NodeState::LeftChild => node = node.get_parent_ptr(),
                    NodeState::RightChild => {
                        return NodeIterator::Node {
                            root,
                            node: node.get_parent_ptr(),
                        };
                    }
                    _ => unreachable!(),
                }
            }

            NodeIterator::INF(root)
        }
        NodeIterator::SUP(root) => NodeIterator::Node {
            root,
            node: get_max(root.clone()),
        },
    }
}

/// 次に大きい値をもつノードを返す
///
/// - 計算量： `O(1) amotized`
pub fn next<K: Ord, V>(iter: NodeIterator<K, V>, root: NodePtr<K, V>) -> NodeIterator<K, V> {
    match iter {
        NodeIterator::INF(root) => NodeIterator::Node {
            root,
            node: get_min(root.clone()),
        },
        NodeIterator::Node { root, mut node } => {
            if let Some(right) = node.right().map(|node| node.clone()) {
                if let Some(mut nxt) = right {
                    while let Some(left) =
                        Some(nxt.clone()).left().map(|node| node.clone()).unwrap()
                    {
                        nxt = left;
                    }
                    return NodeIterator::Node {
                        root,
                        node: Some(nxt),
                    };
                }
            }

            // 親をたどる
            while node.is_child() {
                match node.get_state() {
                    NodeState::RightChild => node = node.get_parent_ptr(),
                    NodeState::LeftChild => {
                        return NodeIterator::Node {
                            root,
                            node: node.get_parent_ptr(),
                        };
                    }
                    _ => unreachable!(),
                }
            }

            NodeIterator::SUP(root)
        }
        NodeIterator::SUP(root) => NodeIterator::SUP(root),
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

#[cfg(test)]
mod test_prev_next {
    use crate::{
        node::{
            insert::insert_single,
            iterator::{get_min, next, prev, NodeIterator},
            pointer::NodeOps,
        },
        print_util::print_as_binary_tree,
    };

    #[test]
    fn test_min() {
        let mut root = None;
        let items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _, _) = insert_single(root, i, i);
        }

        print_as_binary_tree(&root);

        let min = get_min(root.clone());

        assert_eq!(*min.key().unwrap(), -1);
    }

    #[test]
    fn test_prev() {
        let mut root = None;
        let mut items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _, _) = insert_single(root, i, i);
        }

        print_as_binary_tree(&root);

        let mut itr = prev(NodeIterator::SUP(&root), root.clone());
        println!("itr: {:?}", itr);

        // アイテムをソート
        items.sort();

        for i in items.iter().rev() {
            assert_eq!(*itr.as_ref().unwrap().key().unwrap(), *i);

            itr = prev(itr, root.clone());
            println!("itr: {:?}", itr);
        }

        assert!(itr.is_inf());
    }

    #[test]
    fn test_next() {
        let mut root = None;
        let mut items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _, _) = insert_single(root, i, i);
        }

        print_as_binary_tree(&root);

        let mut itr = next(NodeIterator::INF(&root), root.clone());

        // アイテムをソート
        items.sort();

        for i in items {
            assert_eq!(*itr.as_ref().unwrap().key().unwrap(), i);

            itr = next(itr, root.clone());
        }

        assert!(itr.is_sup());
    }
}
