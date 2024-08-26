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
/// - Option\<(K, V)\>: 削除されたノードのキー，値
pub fn remove<K: Ord, V>(
    root: NodePtr<K, V>,
    node: NodePtr<K, V>,
) -> (NodePtr<K, V>, Option<(K, V)>) {
    todo!()
}

/// 葉ノード leaf を削除し，削除済み頂点のポインタを返す
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
        node::insert::{self, insert_single},
        print_util::print_as_binary_tree,
    };

    #[test]
    fn test_remove_leaf() {
        let mut root = None;

        for i in 0..10 {
            (root, _, _) = insert_single(root, i, i);
        }

        print_as_binary_tree(&root);
    }
}
