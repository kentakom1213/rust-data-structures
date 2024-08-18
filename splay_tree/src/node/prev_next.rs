use super::{node_pointer::NodeOps, state::NodeState, NodePtr};

/// 次に小さい値を持つノードを返す
///
/// - 計算量： `O(1) amotized`
pub fn prev<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    if node.is_none() {
        return None;
    }

    // 左の子がいる ⇒ 左の子孫の最大値
    if let Some(mut prev) = node.as_ref()?.borrow().left.clone() {
        while prev.borrow().right.is_some() {
            let right = prev.borrow().right.clone()?;
            prev = right;
        }
        return Some(prev);
    }

    // 親をたどる
    while node.is_child() {
        match node.get_state() {
            NodeState::LeftChild => {
                node = node.get_parent();
            }
            NodeState::RightChild => {
                return node.get_parent();
            }
            _ => unreachable!(),
        }
    }

    None
}

/// 次に大きい値をもつノードを返す
///
/// - 計算量： `O(1) amotized`
pub fn next<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    if node.is_none() {
        return None;
    }

    // 右の子がいる ⇒ 右の子孫の最小値
    if let Some(mut nxt) = node.as_ref()?.borrow().right.clone() {
        while nxt.borrow().left.is_some() {
            let left = nxt.borrow().left.clone()?;
            nxt = left;
        }
        return Some(nxt);
    }

    // 親をたどる
    while node.is_child() {
        match node.get_state() {
            NodeState::LeftChild => {
                return node.get_parent();
            }
            NodeState::RightChild => {
                node = node.get_parent();
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
            insert::{find, insert},
            node_pointer::NodeOps,
            prev_next::{next, prev},
        },
        print_util::print_as_binary_tree,
    };

    #[test]
    fn test_prev() {
        let mut root = None;
        let mut items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _) = insert(root, i, i);
        }

        print_as_binary_tree(&root);

        let mut prv;
        (root, prv) = find(root, &100);

        // アイテムをソート
        items.sort();

        for i in items.iter().rev() {
            assert_eq!(*prv.get_key().unwrap(), *i);

            prv = prev(prv);
        }

        assert!(prv.is_none());
    }

    #[test]
    fn test_next() {
        let mut root = None;
        let mut items = [7, 4, 100, 0, 6, -1, 33, 21];

        for i in items {
            (root, _) = insert(root, i, i);
        }

        print_as_binary_tree(&root);

        let mut nxt;
        (root, nxt) = find(root, &-1);

        // アイテムをソート
        items.sort();

        for i in items {
            assert_eq!(*nxt.get_key().unwrap(), i);

            nxt = next(nxt);
        }

        assert!(nxt.is_none());
    }

    #[test]
    fn test_next2() {}
}
