use std::rc::Rc;

use super::{node_pointer::NodeOps, NodePtr, ParentPtr};

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
        node::{
            insert::{find, insert},
            node_pointer::NodeOps,
            state::NodeState,
        },
        print_util::print_as_binary_tree,
    };

    #[test]
    fn test_nodestate() {
        let mut root = None;
        (root, _) = insert(root, 5, "first");
        (root, _) = insert(root, 15, "second");
        (root, _) = insert(root, 1, "third");
        (root, _) = insert(root, 3, "forth");
        (root, _) = insert(root, 30, "fifth");

        print_as_binary_tree(&root);

        let find_1;
        (root, find_1) = find(root, &1);
        assert_eq!(find_1.get_state(), NodeState::LeftChild);

        let find_3;
        (root, find_3) = find(root, &3);
        assert_eq!(find_3.get_state(), NodeState::RightChild);

        let find_5;
        (root, find_5) = find(root, &5);
        assert_eq!(find_5.get_state(), NodeState::Root);

        let find_15;
        (root, find_15) = find(root, &15);
        assert_eq!(find_15.get_state(), NodeState::RightChild);

        let find_20;
        (root, find_20) = find(root, &20);
        assert_eq!(find_20.get_state(), NodeState::Nil);

        let find_30;
        (root, find_30) = find(root, &30);
        assert_eq!(find_30.get_state(), NodeState::RightChild);

        (root, _) = insert(root, 20, "sixth");
        print_as_binary_tree(&root);

        let find_20;
        (root, find_20) = find(root, &20);
        assert_eq!(find_20.get_state(), NodeState::LeftChild);
    }
}
