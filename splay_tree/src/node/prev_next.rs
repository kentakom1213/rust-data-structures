use super::{
    state::{get_parent, NodeState},
    NodePtr,
};

/// 次に大きい値をもつノードを返す
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
    let mut state = NodeState::get(&node);
    while state.is_child() {
        match state {
            NodeState::LeftChild => {
                return get_parent(&node);
            }
            NodeState::RightChild => {
                node = get_parent(&node);
            }
            _ => unreachable!(),
        }
        state = NodeState::get(&node);
    }

    None
}

#[cfg(test)]
mod test_prev_next {
    use crate::{
        node::{
            insert::{find, insert},
            prev_next::next,
        },
        print_util::print_as_binary_tree,
    };

    #[test]
    fn test_next() {
        let mut root = None;

        for i in [7, 4, 100, 0, 6, -1, 33, 21] {
            (root, _) = insert(root, i, i);
        }

        print_as_binary_tree(&root);

        let mut nxt;
        (root, nxt) = find(root, &-1);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");
    }
}
