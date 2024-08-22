use std::{cmp::Ordering, fmt::Debug};

use super::{
    node_pointer::{Node, NodeOps},
    NodePtr,
};

/// rootを根とする木に(key, value)を挿入する．
/// 挿入後のノードの参照を返す．
/// すでに同一のキーを持つノードが存在した場合，値を置き換える．
///
/// **引数**
/// - node: 挿入対象のノード
/// - key: キー
/// - value: 値
///
/// **戻り値**
/// - NodePtr<K, V>: 挿入後の根ノード
/// - NodePtr<K, V>: 追加されたノード
/// - Option<V>: 置き換えられた値
pub fn insert_single<K: Ord + Debug, V: Clone + Debug>(
    root: NodePtr<K, V>,
    key: K,
    value: V,
) -> (NodePtr<K, V>, NodePtr<K, V>) {
    if root.is_none() {
        let new_node = Node::node_ptr(key, value);
        return (new_node.clone(), new_node);
    }

    // 親ノードをたどっていく
    let mut par = root.clone();

    loop {
        let comp = key.cmp(&par.key().unwrap());
        match comp {
            Ordering::Less => {
                if let Some(left) = par.left().map(|node| node.clone()).unwrap() {
                    par = Some(left);
                } else {
                    // 左側に挿入
                    return (root, insert_left(par, key, value));
                }
            }
            Ordering::Equal => {
                // 置き換える
                *par.value_mut().unwrap() = value;
                return (root, par);
            }
            Ordering::Greater => {
                if let Some(right) = par.right().map(|node| node.clone()).unwrap() {
                    par = Some(right);
                } else {
                    // 右側に挿入
                    return (root, insert_right(par, key, value));
                }
            }
        }
    }
}

/// nodeの左側に子を追加し，追加された子のポインタを返す
fn insert_left<K: Ord, V>(mut node: NodePtr<K, V>, key: K, value: V) -> NodePtr<K, V> {
    let mut new_node = Node::node_ptr(key, value);

    // node.left.parent ← new_node
    if let Some(mut left) = node.left_mut() {
        if let Some(mut left_par) = left.parent_mut() {
            *left_par = new_node.to_weak_ptr();
        };
    }

    // new_node.left ← node.left
    *new_node.left_mut().unwrap() = node.take_left();

    // new_node.parent ← node
    *new_node.parent_mut().unwrap() = node.to_weak_ptr();

    // node.left ← new_node
    if let Some(mut left) = node.left_mut() {
        *left = new_node.clone();
    }

    new_node
}

/// nodeの右側に子を追加し，追加された子のポインタを返す
fn insert_right<K: Ord, V>(mut node: NodePtr<K, V>, key: K, value: V) -> NodePtr<K, V> {
    let mut new_node = Node::node_ptr(key, value);

    // node.right.parent ← new_node
    if let Some(mut right) = node.right_mut() {
        if let Some(mut right_par) = right.parent_mut() {
            *right_par = new_node.to_weak_ptr();
        };
    }

    // new_node.right ← node.right
    *new_node.right_mut().unwrap() = node.take_right();

    // new_node.parent ← node
    *new_node.parent_mut().unwrap() = node.to_weak_ptr();

    // node.right ← new_node
    if let Some(mut right) = node.right_mut() {
        *right = new_node.clone();
    }

    new_node
}

#[cfg(test)]
mod test_insert {
    use crate::{
        node::{insert::insert_right, node_pointer::NodeOps, splay::splay},
        print_util::print_as_binary_tree,
    };

    use super::{insert_left, insert_single};

    #[test]
    fn test_insert_left() {
        let mut root = None;

        {
            let res = insert_left(root.clone(), 3, "first");
            println!("{:?}", res);

            root = res;
        }

        print_as_binary_tree(&root);

        {
            let res = insert_left(root.clone(), 1, "second");
            println!("{:?}", res);
        }

        print_as_binary_tree(&root);

        {
            let res = insert_left(root.clone(), 2, "third");
            println!("{:?}", res);
        }

        print_as_binary_tree(&root);
    }

    #[test]
    fn test_insert_right() {
        let mut root = None;

        {
            let res = insert_right(root.clone(), 3, "first");
            println!("{:?}", res);

            root = res;
        }

        print_as_binary_tree(&root);

        {
            let res = insert_right(root.clone(), 5, "second");
            println!("{:?}", res);
        }

        print_as_binary_tree(&root);

        {
            let res = insert_right(root.clone(), 4, "third");
            println!("{:?}", res);
        }

        print_as_binary_tree(&root);
    }

    #[test]
    fn test_insert_single() {
        let mut root = None;
        print_as_binary_tree(&root);

        {
            let new_node;
            (root, new_node) = insert_single(root, 5, "first");
            println!("new_node: {new_node:?}");
            print_as_binary_tree(&root);
        }

        {
            let new_node;
            (root, new_node) = insert_single(root, 15, "second");
            println!("new_node: {new_node:?}");
            print_as_binary_tree(&root);
        }

        {
            let new_node;
            (root, new_node) = insert_single(root, 1, "third");
            println!("new_node: {new_node:?}");
            print_as_binary_tree(&root);
        }

        {
            let new_node;
            (root, new_node) = insert_single(root, 3, "forth");
            println!("new_node: {new_node:?}");
            print_as_binary_tree(&root);
        }

        {
            let new_node;
            (root, new_node) = insert_single(root, 30, "fifth");
            println!("new_node: {new_node:?}");
            print_as_binary_tree(&root);
        }

        {
            let new_node;
            (root, new_node) = insert_single(root, 15, "sixth");
            println!("new_node: {new_node:?}");
            print_as_binary_tree(&root);
        }
    }

    #[test]
    fn test_insert_single2() {
        let mut root = None;

        for i in 0..=20 {
            (root, _) = insert_single(root, i, i.to_string());
        }

        print_as_binary_tree(&root);

        let dup;
        (root, dup) = insert_single(root, 20, "Updated".to_string());

        assert_eq!(dup.value().unwrap().clone(), "Updated".to_string());

        print_as_binary_tree(&root);
    }
}
