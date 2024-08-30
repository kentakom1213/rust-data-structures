use crate::node::state::NodeState;

use super::{
    pointer::{NodeOps, ParentOps},
    NodePtr,
};

/// ノード node を削除する
///
/// **引数**
/// - root: 削除対象の木の根のポインタ
/// - node: 削除対象のノードのポインタ
///
/// **戻り値**
/// - NodePtr\<K, V\>: 削除後の木の根のポインタ
/// - NodePtr\<K, V\>: 削除されたノードのポインタ
pub fn remove<K: Ord, V>(
    mut root: NodePtr<K, V>,
    mut node: NodePtr<K, V>,
) -> (NodePtr<K, V>, NodePtr<K, V>) {
    let state = node.get_state();
    let left = node.take_left();
    let right = node.take_right();
    let parent = node.take_parent_strong();

    // 葉になったので自分を削除
    let node = remove_leaf(node);

    match (left.is_some(), right.is_some()) {
        (false, false) => match state {
            NodeState::Root => root = None,
            _ => (),
        },
        (false, true) => match state {
            NodeState::Root => (root, node) = (right, node),
        },
        (true, false) => todo!(),
        (true, true) => todo!(),
    }

    (root, node)
}

/// 葉ノード leaf を削除し，削除済み頂点のポインタを返す
///
/// （葉ノードであるかの判定は行わない）
fn remove_leaf<K: Ord, V>(mut leaf: NodePtr<K, V>) -> NodePtr<K, V> {
    // 親ノードから切り離す
    let mut node = match leaf.get_state() {
        NodeState::Nil => return None,
        NodeState::Root => leaf,
        NodeState::LeftChild => leaf.get_parent_ptr().take_left(),
        NodeState::RightChild => leaf.get_parent_ptr().take_right(),
    };

    // 親を削除
    node.take_parent();

    node
}

#[cfg(test)]
mod test_remove {
    use crate::{
        node::{find::find, insert::insert_single, pointer::NodeOps},
        print_util::print_as_binary_tree,
    };

    use super::remove_leaf;

    #[test]
    fn test_remove_leaf() {
        let mut root = None;

        for i in [4, 2, 6, 1, 3, 5, 6, 7] {
            (root, _, _) = insert_single(root, i, i);
        }

        print_as_binary_tree(&root);

        for i in [1, 3, 5, 7] {
            let node = find(root.clone(), &i);

            println!("{:?}", node.get_state());

            let node = remove_leaf(node);

            println!("{:?}", node.get_state());

            print_as_binary_tree(&root);
        }
    }
}
