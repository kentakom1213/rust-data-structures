use std::fmt::Debug;

use crate::node::state::NodeState;

use super::{iterator::get_min, pointer::NodeOps, NodePtr};

/// ノード node を削除する
///
/// **引数**
/// - root: 削除対象の木の根のポインタ
/// - node: 削除対象のノードのポインタ
///
/// **戻り値**
/// - NodePtr\<K, V\>: 削除後の木の根のポインタ
/// - NodePtr\<K, V\>: 削除後，代わりに埋められたノードのポインタ
/// - NodePtr\<K, V\>: 削除されたノードのポインタ
pub fn remove<K: Ord + Debug, V: Debug>(
    mut root: NodePtr<K, V>,
    mut node: NodePtr<K, V>,
) -> (NodePtr<K, V>, NodePtr<K, V>, NodePtr<K, V>) {
    let state = node.get_state();

    // 削除対象のノードがない場合
    if matches!(state, NodeState::Nil) {
        return (root, None, None);
    }

    // 子のポインタを取り出す
    let left = node.take_left();
    let right = node.take_right();

    // 葉になったので自分を削除
    let mut parent;
    let removed_node;
    (root, parent, removed_node) = remove_leaf(root, node);

    let new_node;

    match (left, right) {
        (None, None) => {
            match state {
                NodeState::Root => {}
                NodeState::LeftChild => *parent.left_mut().unwrap() = None,
                NodeState::RightChild => *parent.right_mut().unwrap() = None,
                NodeState::Nil => unreachable!(),
            }
            new_node = None;
        }
        (mut left @ Some(_), None) => {
            match state {
                NodeState::Root => root = left.clone(),
                NodeState::LeftChild => {
                    *left.parent_mut().unwrap() = parent.to_weak_ptr();
                    *parent.left_mut().unwrap() = left.clone();
                }
                NodeState::RightChild => {
                    *left.parent_mut().unwrap() = parent.to_weak_ptr();
                    *parent.right_mut().unwrap() = left.clone();
                }
                NodeState::Nil => unreachable!(),
            };
            new_node = left;
        }
        (None, mut right @ Some(_)) => {
            match state {
                NodeState::Root => root = right.clone(),
                NodeState::LeftChild => {
                    *right.parent_mut().unwrap() = parent.to_weak_ptr();
                    *parent.left_mut().unwrap() = right.clone();
                }
                NodeState::RightChild => {
                    *right.parent_mut().unwrap() = parent.to_weak_ptr();
                    *parent.right_mut().unwrap() = right.clone();
                }
                NodeState::Nil => unreachable!(),
            }
            new_node = right;
        }
        (mut left @ Some(_), mut right @ Some(_)) => {
            // 右の子の最小値を取り出す
            let mut right_min = get_min(right.clone());
            (right, _, right_min) = remove_leaf(right, right_min);

            *left.parent_mut().unwrap() = right_min.to_weak_ptr();
            if let Some(mut right_par) = right.parent_mut() {
                *right_par = right_min.to_weak_ptr();
            }
            *right_min.left_mut().unwrap() = left;
            *right_min.right_mut().unwrap() = right;

            match state {
                NodeState::Root => root = right_min.clone(),
                NodeState::LeftChild => {
                    *right_min.parent_mut().unwrap() = parent.to_weak_ptr();
                    *parent.left_mut().unwrap() = right_min.clone();
                }
                NodeState::RightChild => {
                    *right_min.parent_mut().unwrap() = parent.to_weak_ptr();
                    *parent.right_mut().unwrap() = right_min.clone();
                }
                NodeState::Nil => unreachable!(),
            };

            new_node = right_min;
        }
    }

    (root, new_node, removed_node)
}

/// 葉ノード leaf を削除し，削除済み頂点のポインタを返す
///
/// **引数**
/// - root: 削除対象の木の根のポインタ
/// - node: 削除対象のノードのポインタ
///
/// **戻り値**
/// - NodePtr\<K, V\>: 削除後の木の根のポインタ
/// - NodePtr\<K, V\>: 削除前の葉の親のポインタ
/// - NodePtr\<K, V\>: 削除された葉ノードのポインタ
fn remove_leaf<K: Ord + Debug, V: Debug>(
    root: NodePtr<K, V>,
    leaf: NodePtr<K, V>,
) -> (NodePtr<K, V>, NodePtr<K, V>, NodePtr<K, V>) {
    if root.is_same(&leaf) {
        return (None, None, leaf);
    }

    // 親ノード
    let mut parent = leaf.get_parent_ptr();

    // 親ノードから切り離す
    let removed_node = match leaf.get_state() {
        NodeState::Nil => return (root, None, None),
        NodeState::Root => return (None, None, leaf),
        NodeState::LeftChild => parent.take_left(),
        NodeState::RightChild => parent.take_right(),
    };

    (root, parent, removed_node)
}

#[cfg(test)]
mod test_remove {
    use crate::{
        node::{find::find, insert::insert_single, pointer::NodeOps},
        print_util::print_as_binary_tree,
    };

    use super::{remove, remove_leaf};

    #[test]
    fn test_remove_leaf() {
        let mut root = None;

        for i in [4, 2, 6, 1, 3, 5, 7] {
            (root, _, _) = insert_single(root, i, i);
        }

        print_as_binary_tree(&root);

        for i in [1, 3, 5, 7, 2, 6, 4] {
            let node = find(root.clone(), &i);

            println!("Remove {i}");

            let removed_node;
            (root, _, removed_node) = remove_leaf(root, node);

            println!("removed_node: {:?}", removed_node);

            print_as_binary_tree(&root);
        }
    }

    #[test]
    fn test_remove_inner() {
        let mut root = None;

        for i in [4, 2, 6, 1, 3, 5, 7] {
            (root, _, _) = insert_single(root, i, i);
        }

        println!("Initial Tree");
        print_as_binary_tree(&root);

        {
            let node = find(root.clone(), &7);
            let new_node;
            let removed_node;
            (root, new_node, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 7);
            assert!(find(root.clone(), &7).is_none());
        }

        println!("Remove 7");
        print_as_binary_tree(&root);

        {
            let node = find(root.clone(), &6);
            let new_node;
            let removed_node;
            (root, new_node, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 6);
            assert!(find(root.clone(), &6).is_none());
        }

        println!("Remove 6");
        print_as_binary_tree(&root);

        {
            let node = find(root.clone(), &7);
            let new_node;
            let removed_node;
            (root, new_node, removed_node) = remove(root, node);
            assert!(removed_node.is_none());
            assert!(find(root.clone(), &7).is_none());
        }

        println!("Remove 7");
        print_as_binary_tree(&root);

        {
            let node = find(root.clone(), &4);
            let new_node;
            let removed_node;
            (root, new_node, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 4);
            assert!(find(root.clone(), &4).is_none());
        }

        println!("Remove 4");
        print_as_binary_tree(&root);

        {
            let node = find(root.clone(), &2);
            let new_node;
            let removed_node;
            (root, new_node, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 2);
            assert!(find(root.clone(), &2).is_none());
        }

        println!("Remove 2");
        print_as_binary_tree(&root);
    }

    #[test]
    fn test_small() {
        let mut root = None;
        (root, _, _) = insert_single(root, 1, 1);
        (root, _, _) = insert_single(root, 0, 0);
        (root, _, _) = insert_single(root, 3, 3);

        print_as_binary_tree(&root);

        let removed;
        let node = find(root.clone(), &1);
        (root, _, removed) = remove(root, node);

        assert_eq!(*removed.key().unwrap(), 1);

        print_as_binary_tree(&root);
    }
}
