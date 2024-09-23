use std::fmt::Debug;

use super::{iterator::get_min, pointer::NodeOps, splay::splay, NodePtr};

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
    node: NodePtr<K, V>,
) -> (NodePtr<K, V>, NodePtr<K, V>) {
    // nodeが存在しない場合
    if node.is_none() {
        return (root, node);
    }

    // nodeを根に持ってくる
    root = splay(node);

    // 左右に分割
    let mut left = root.take_left();
    let mut right = root.take_right();

    // 右部分木の最小値を取得
    let right_min = get_min(right.clone());

    right = splay(right_min);

    // right.left <- left
    if let Some(mut left_par) = left.parent_mut() {
        *left_par = right.to_weak_ptr();
    }
    if let Some(mut right_left) = right.left_mut() {
        *right_left = left;
    } else {
        return (left, root);
    }

    (right, root)
}

#[cfg(test)]
mod test_remove {
    use crate::{
        node::{find::find, insert::insert_single, pointer::NodeOps},
        print_util::print_as_binary_tree,
    };

    use super::remove;

    #[test]
    fn test_remove_inner() {
        let mut root = None;

        for i in [4, 2, 6, 1, 3, 5, 7] {
            (root, _, _) = insert_single(root, i, i);
        }

        println!("Initial Tree");
        print_as_binary_tree(&root);

        println!("Remove 7");
        {
            let node = find(root.clone(), &7);
            let removed_node;
            (root, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 7);
            assert!(find(root.clone(), &7).is_none());
        }
        print_as_binary_tree(&root);

        println!("Remove 6");
        {
            let node = find(root.clone(), &6);
            let removed_node;
            (root, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 6);
            assert!(find(root.clone(), &6).is_none());
        }
        print_as_binary_tree(&root);

        println!("Remove 7");
        {
            let node = find(root.clone(), &7);
            let removed_node;
            (root, removed_node) = remove(root, node);
            assert!(removed_node.is_none());
            assert!(find(root.clone(), &7).is_none());
        }
        print_as_binary_tree(&root);

        println!("Remove 4");
        {
            let node = find(root.clone(), &4);
            let removed_node;
            (root, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 4);
            assert!(find(root.clone(), &4).is_none());
        }
        print_as_binary_tree(&root);

        println!("Remove 2");
        {
            let node = find(root.clone(), &2);
            let removed_node;
            (root, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 2);
            assert!(find(root.clone(), &2).is_none());
        }
        print_as_binary_tree(&root);

        println!("Remove 1");
        {
            let node = find(root.clone(), &1);
            let removed_node;
            (root, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 1);
            assert!(find(root.clone(), &1).is_none());
        }
        print_as_binary_tree(&root);

        println!("Remove 3");
        {
            let node = find(root.clone(), &3);
            let removed_node;
            (root, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 3);
            assert!(find(root.clone(), &3).is_none());
        }
        print_as_binary_tree(&root);

        println!("Remove 5");
        {
            let node = find(root.clone(), &5);
            let removed_node;
            (root, removed_node) = remove(root, node);
            assert_eq!(*removed_node.key().unwrap(), 5);
            assert!(find(root.clone(), &5).is_none());
        }
        print_as_binary_tree(&root);

        assert!(root.is_none());
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
        (root, removed) = remove(root, node);

        assert_eq!(*removed.key().unwrap(), 1);

        print_as_binary_tree(&root);
    }
}
