use std::fmt::Debug;

use super::{pointer::NodeOps, state::NodeState, NodePtr};

/// 次に小さい値を持つノードを返す
///
/// - 計算量： `O(1) amotized`
pub fn prev<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    if let Some(mut prv) = node.left().map(|node| node.clone())? {
        while let Some(right) = Some(prv.clone()).right().map(|node| node.clone())? {
            prv = right;
        }
        return Some(prv);
    }

    // 親をたどる
    while node.is_child() {
        match node.get_state() {
            NodeState::LeftChild => {
                node = node.get_parent_ptr();
            }
            NodeState::RightChild => {
                return node.get_parent_ptr();
            }
            _ => unreachable!(),
        }
    }

    None
}

/// 次に大きい値をもつノードを返す
///
/// - 計算量： `O(1) amotized`
pub fn next<K: Ord + Debug, V: Debug>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    if let Some(mut nxt) = node.right().map(|node| node.clone())? {
        while let Some(left) = Some(nxt.clone()).left().map(|node| node.clone())? {
            nxt = left;
        }
        return Some(nxt);
    }

    // 親をたどる
    while node.is_child() {
        match node.get_state() {
            NodeState::LeftChild => {
                return node.get_parent_ptr();
            }
            NodeState::RightChild => {
                node = node.get_parent_ptr();
            }
            _ => unreachable!(),
        }
    }

    None
}

#[cfg(test)]
mod test_prev_next {
    use crate::{
        node::{
            find::lower_bound,
            insert::insert_single,
            pointer::NodeOps,
            iterator::{next, prev},
        },
        print_util::print_as_binary_tree,
    };

    #[test]
    fn test_prev() {
        let mut root = None;
        let mut items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _, _) = insert_single(root, i, i);
        }

        print_as_binary_tree(&root);

        let mut prv = lower_bound(root.clone(), &100);

        // アイテムをソート
        items.sort();

        for i in items.iter().rev() {
            assert_eq!(*prv.key().unwrap(), *i);

            prv = prev(prv);
        }

        assert!(prv.is_none());
    }

    #[test]
    fn test_next() {
        let mut root = None;
        let mut items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _, _) = insert_single(root, i, i);
        }

        print_as_binary_tree(&root);

        let mut nxt = lower_bound(root.clone(), &-1);

        // アイテムをソート
        items.sort();

        for i in items {
            assert_eq!(*nxt.key().unwrap(), i);

            nxt = next(nxt);
        }

        assert!(nxt.is_none());
    }

    #[test]
    fn test_next2() {
        let mut root = None;

        (root, _, _) = insert_single(root, 1, "first");
        (root, _, _) = insert_single(root, 2, "second");

        print_as_binary_tree(&root);

        let mut nxt = lower_bound(root.clone(), &1);

        assert_eq!(*nxt.key().unwrap(), 1);

        nxt = next(nxt);
        assert_eq!(*nxt.key().unwrap(), 2);
    }
}
