use crate::node::{
    iterator::get_min,
    pointer::{NodeOps, NodePtr},
    splay::splay,
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
pub fn remove<K: Ord, V>(mut node: NodePtr<K, V>) -> (Option<NodePtr<K, V>>, NodePtr<K, V>) {
    // nodeを根に持ってくる
    let root = splay(node.clone());

    // 左右に分割
    let left = node.take_left();
    let mut right = node.take_right();

    // 右部分木の最小値を取得
    let right_min = get_min(right.clone());

    if let Some(right_min_inner) = right_min {
        right = Some(splay(right_min_inner));
    }

    // right.left <- left
    if let Some(mut left_inner) = left.clone() {
        *left_inner.parent_mut() = right.clone().map(|ptr| ptr.to_weak_ptr());
    }
    if let Some(mut right_inner) = right.clone() {
        *right_inner.left_mut() = left;
    } else {
        return (left, root);
    }

    (right, root)
}

#[cfg(test)]
mod test_remove {
    use crate::{
        node::{find::find, insert::insert, pointer::NodeOps},
        utils::print::print_as_tree,
    };

    use super::remove;

    #[test]
    fn test_remove_inner() {
        let mut root = None;

        for i in [4, 2, 6, 1, 3, 5, 7] {
            (root, _, _) = insert(root, i, i);
        }

        println!("Initial Tree");
        print_as_tree(&root);

        println!("Remove 7");
        {
            let node;
            (root, node) = find(root.clone(), &7);
            let removed_node;
            (root, removed_node) = remove(node.unwrap());
            assert_eq!(*removed_node.key(), 7);

            let node;
            (root, node) = find(root.clone(), &7);
            assert!(node.is_none());
        }
        print_as_tree(&root);

        println!("Remove 6");
        {
            let node;
            (root, node) = find(root.clone(), &6);
            let removed_node;
            (root, removed_node) = remove(node.unwrap());
            assert_eq!(*removed_node.key(), 6);

            let node;
            (root, node) = find(root.clone(), &6);
            assert!(node.is_none());
        }
        print_as_tree(&root);

        println!("Remove 7");
        {
            let node;
            (root, node) = find(root.clone(), &7);
            assert!(node.is_none());

            let node;
            (root, node) = find(root.clone(), &7);
            assert!(node.is_none());
        }
        print_as_tree(&root);

        println!("Remove 4");
        {
            let node;
            (root, node) = find(root.clone(), &4);
            let removed_node;
            (root, removed_node) = remove(node.unwrap());
            assert_eq!(*removed_node.key(), 4);

            let node;
            (root, node) = find(root.clone(), &4);
            assert!(node.is_none());
        }
        print_as_tree(&root);

        println!("Remove 2");
        {
            let node;
            (root, node) = find(root.clone(), &2);
            let removed_node;
            (root, removed_node) = remove(node.unwrap());
            assert_eq!(*removed_node.key(), 2);

            let node;
            (root, node) = find(root.clone(), &2);
            assert!(node.is_none());
        }
        print_as_tree(&root);

        println!("Remove 1");
        {
            let node;
            (root, node) = find(root.clone(), &1);
            let removed_node;
            (root, removed_node) = remove(node.unwrap());
            assert_eq!(*removed_node.key(), 1);

            let node;
            (root, node) = find(root.clone(), &1);
            assert!(node.is_none());
        }
        print_as_tree(&root);

        println!("Remove 3");
        {
            let node;
            (root, node) = find(root.clone(), &3);
            let removed_node;
            (root, removed_node) = remove(node.unwrap());
            assert_eq!(*removed_node.key(), 3);

            let node;
            (root, node) = find(root.clone(), &3);
            assert!(node.is_none());
        }
        print_as_tree(&root);

        println!("Remove 5");
        {
            let node;
            (root, node) = find(root.clone(), &5);
            let removed_node;
            (root, removed_node) = remove(node.unwrap());
            assert_eq!(*removed_node.key(), 5);

            let node;
            (root, node) = find(root.clone(), &5);
            assert!(node.is_none());
        }
        print_as_tree(&root);

        assert!(root.is_none());
    }

    #[test]
    fn test_small() {
        let mut root = None;
        (root, _, _) = insert(root, 1, 1);
        (root, _, _) = insert(root, 0, 0);
        (root, _, _) = insert(root, 3, 3);

        print_as_tree(&root);

        let removed;
        let node;
        (root, node) = find(root, &0);

        print_as_tree(&root);

        (root, removed) = remove(node.unwrap());

        assert_eq!(*removed.key(), 0);

        print_as_tree(&root);
    }
}
