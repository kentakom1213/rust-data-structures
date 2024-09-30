//! ノードの状態を返す列挙子

/// ノードの状態を調べる
#[derive(Debug, PartialEq)]
pub enum NodeState {
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
        node::{insert::insert, pointer::NodeOps, state::NodeState},
        print_util::print_as_tree,
    };

    #[test]
    fn test_nodestate() {
        let (find_1, find_3, find_5, find_15, find_30);
        let find_20;

        let mut root = None;
        (root, find_5, _) = insert(root, 5, "first");
        (root, find_15, _) = insert(root, 15, "second");
        (root, find_1, _) = insert(root, 1, "third");
        (root, find_3, _) = insert(root, 3, "forth");
        (root, find_30, _) = insert(root, 30, "fifth");

        print_as_tree(&root);
        assert_eq!(find_1.get_state(), NodeState::LeftChild);
        assert_eq!(find_3.get_state(), NodeState::RightChild);
        assert_eq!(find_5.get_state(), NodeState::Root);
        assert_eq!(find_15.get_state(), NodeState::RightChild);
        assert_eq!(find_30.get_state(), NodeState::RightChild);

        (root, find_20, _) = insert(root, 20, "sixth");
        print_as_tree(&root);
        assert_eq!(find_20.get_state(), NodeState::LeftChild);
    }
}
