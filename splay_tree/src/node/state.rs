//! ノードの状態を返す列挙子

/// ノードの状態を調べる
#[derive(Debug, PartialEq)]
pub enum NodeState {
    /// ノードが存在しない
    Nil,
    /// 根ノード（親を持たない）
    Root,
    /// 親の左の子
    LeftChild,
    /// 親の右の子
    RightChild,
}

#[cfg(test)]
mod test_node_state {
    use crate::{
        node::{find::find, insert::insert_single, node_pointer::NodeOps, state::NodeState},
        print_util::print_as_binary_tree,
    };

    #[test]
    fn test_nodestate() {
        let mut root = None;
        (root, _) = insert_single(root, 5, "first");
        (root, _) = insert_single(root, 15, "second");
        (root, _) = insert_single(root, 1, "third");
        (root, _) = insert_single(root, 3, "forth");
        (root, _) = insert_single(root, 30, "fifth");

        print_as_binary_tree(&root);

        let find_1 = find(root.clone(), &1);
        assert_eq!(find_1.get_state(), NodeState::LeftChild);

        let find_3 = find(root.clone(), &3);
        assert_eq!(find_3.get_state(), NodeState::RightChild);

        let find_5 = find(root.clone(), &5);
        assert_eq!(find_5.get_state(), NodeState::Root);

        let find_15 = find(root.clone(), &15);
        assert_eq!(find_15.get_state(), NodeState::RightChild);

        let find_20 = find(root.clone(), &20);
        assert_eq!(find_20.get_state(), NodeState::Nil);

        let find_30 = find(root.clone(), &30);
        assert_eq!(find_30.get_state(), NodeState::RightChild);

        (root, _) = insert_single(root, 20, "sixth");
        print_as_binary_tree(&root);

        let find_20 = find(root.clone(), &20);
        assert_eq!(find_20.get_state(), NodeState::LeftChild);
    }
}
