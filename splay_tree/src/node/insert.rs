use std::{cmp::Ordering, mem};

use super::pointer::NodePtr;
use super::pointer::{Node, NodeOps};

/// rootを根とする木に(key, value)を挿入し，挿入後のノードの参照を返す．
/// すでに同一のキーを持つノードが存在した場合，値を置き換える．
///
/// **引数**
/// - node: 挿入対象のノード
/// - key: キー
/// - value: 値
///
/// **戻り値**
/// - Option<NodePtr<K, V>>: 挿入後の根ノード
/// - Option<NodePtr<K, V>>: 追加されたノード
/// - Option<V>: 置き換えられた値
pub fn insert<K: Ord, V>(
    root: Option<NodePtr<K, V>>,
    key: K,
    value: V,
) -> (Option<NodePtr<K, V>>, NodePtr<K, V>, Option<V>) {
    if root.is_none() {
        let new_root = Node::node_ptr(key, value);
        return (Some(new_root.clone()), new_root, None);
    }

    // 親ノードをたどっていく
    let mut par = root.clone();

    loop {
        let comp = key.cmp(&par.as_ref().unwrap().key());
        match comp {
            Ordering::Less => {
                if let Some(Some(left)) = par.as_ref().map(|node| node.left().clone()) {
                    par.replace(left);
                } else {
                    // 左側に挿入
                    break (root, insert_left(par, key, value), None);
                }
            }
            Ordering::Equal => {
                // 置き換える
                let old_value = mem::replace(&mut *par.as_mut().unwrap().value_mut(), value);
                break (root, par.unwrap(), Some(old_value));
            }
            Ordering::Greater => {
                if let Some(Some(right)) = par.as_ref().map(|node| node.right().clone()) {
                    par.replace(right);
                } else {
                    // 右側に挿入
                    break (root, insert_right(par, key, value), None);
                }
            }
        }
    }
}

/// nodeの左側に子を追加し，追加された子のポインタを返す
pub fn insert_left<K: Ord, V>(node: Option<NodePtr<K, V>>, key: K, value: V) -> NodePtr<K, V> {
    let mut new_node = Node::node_ptr(key, value);

    let Some(mut inner) = node else {
        return new_node;
    };

    // new_node.left ← node.left
    *new_node.left_mut() = inner.take_left();

    // left.parent ← new_node
    let new_node_weak = new_node.to_weak_ptr();
    if let Some(left) = new_node.left_mut().as_mut() {
        *left.parent_mut() = Some(new_node_weak);
    }

    // new_node.parent ← node
    *new_node.parent_mut() = Some(inner.to_weak_ptr());

    // node.left ← new_node
    inner.left_mut().replace(new_node.clone());

    new_node
}

/// nodeの右側に子を追加し，追加された子のポインタを返す
pub fn insert_right<K: Ord, V>(node: Option<NodePtr<K, V>>, key: K, value: V) -> NodePtr<K, V> {
    let mut new_node = Node::node_ptr(key, value);

    let Some(mut inner) = node else {
        return new_node;
    };

    // new_node.right ← node.right
    *new_node.right_mut() = inner.take_right();

    // right.parent ← new_node
    let new_node_weak = new_node.to_weak_ptr();
    if let Some(right) = new_node.right_mut().as_mut() {
        *right.parent_mut() = Some(new_node_weak);
    }

    // new_node.parent ← node
    *new_node.parent_mut() = Some(inner.to_weak_ptr());

    // node.right ← new_node
    inner.right_mut().replace(new_node.clone());

    new_node
}

#[cfg(test)]
mod test_insert {
    use crate::{node::pointer::NodeOps, print_util::print_as_tree};

    use super::{insert, insert_left, insert_right};

    #[test]
    fn test_insert_left() {
        let mut root = None;

        {
            let res = insert_left(root.clone(), 3, "first");
            println!("{:?}", res);

            root = Some(res);
        }

        print_as_tree(&root);

        {
            let res = insert_left(root.clone(), 1, "second");
            println!("{:?}", res);
        }

        print_as_tree(&root);

        {
            let res = insert_left(root.clone(), 2, "third");
            println!("{:?}", res);
        }

        print_as_tree(&root);
    }

    #[test]
    fn test_insert_right() {
        let mut root = None;

        {
            let res = insert_right(root.clone(), 3, "first");
            println!("{:?}", res);

            root = Some(res);
        }

        print_as_tree(&root);

        {
            let res = insert_right(root.clone(), 5, "second");
            println!("{:?}", res);
        }

        print_as_tree(&root);

        {
            let res = insert_right(root.clone(), 4, "third");
            println!("{:?}", res);
        }

        print_as_tree(&root);
    }

    #[test]
    fn test_insert() {
        let mut root = None;
        print_as_tree(&root);

        {
            let new_node;
            (root, new_node, _) = insert(root.clone(), 5, "first");
            println!("new_node: {new_node:?}");
            print_as_tree(&root);
        }

        {
            let new_node;
            (root, new_node, _) = insert(root.clone(), 15, "second");
            println!("new_node: {new_node:?}");
            print_as_tree(&root);
        }

        {
            let new_node;
            (root, new_node, _) = insert(root.clone(), 1, "third");
            println!("new_node: {new_node:?}");
            print_as_tree(&root);
        }

        {
            let new_node;
            (root, new_node, _) = insert(root.clone(), 3, "forth");
            println!("new_node: {new_node:?}");
            print_as_tree(&root);
        }

        {
            let new_node;
            (root, new_node, _) = insert(root.clone(), 30, "fifth");
            println!("new_node: {new_node:?}");
            print_as_tree(&root);
        }

        {
            let new_node;
            (root, new_node, _) = insert(root, 15, "sixth");
            println!("new_node: {new_node:?}");
            print_as_tree(&root);
        }
    }

    #[test]
    fn test_insert2() {
        let mut root = None;

        for i in 0..=20 {
            (root, _, _) = insert(root, i, i.to_string());
        }

        print_as_tree(&root);

        let dup;
        (root, dup, _) = insert(root, 20, "Updated".to_string());

        assert_eq!(dup.value().clone(), "Updated".to_string());

        print_as_tree(&root);
    }
}
