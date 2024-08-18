use std::{cmp::Ordering, fmt::Debug, mem, rc::Rc};

use super::{
    node_pointer::{Node, NodeOps},
    NodePtr,
};

/// nodeを根とする木に(key, value)を挿入する
/// - すでに同じキーが存在した場合，その値を置き換える
///
/// **引数**
/// - node: 挿入対象のノード
/// - key: キー
/// - value: 値
///
/// **戻り値**
/// - NodePtr<K, V>: 挿入後のノード
/// - Option<V>: 置き換えられた値
pub fn insert<K: Ord, V: Clone>(
    node: NodePtr<K, V>,
    key: K,
    value: V,
) -> (NodePtr<K, V>, Option<V>) {
    let Some(node) = node else {
        return (Node::node_ptr(key, value), None);
    };

    // キーの比較
    let comp = key.cmp(&node.as_ref().borrow().key);

    match comp {
        Ordering::Less => {
            // 左の子に挿入
            let left = node.borrow_mut().left.take();
            let (mut new_left, old_value) = insert(left, key, value);

            // new_leftの親の更新
            let node_ptr_weak = Rc::downgrade(&node);
            new_left.as_mut().unwrap().borrow_mut().parent = Some(node_ptr_weak);

            // 子を戻す
            node.borrow_mut().left = new_left;

            (Some(node), old_value)
        }
        Ordering::Equal => {
            // valueを置き換える
            let old_value = mem::replace(&mut node.borrow_mut().value, value);

            (Some(node), Some(old_value))
        }
        Ordering::Greater => {
            // 左の子に挿入
            let right = node.borrow_mut().right.take();
            let (mut new_right, old_value) = insert(right, key, value);

            // new_rightの親の更新
            let node_ptr_weak = Rc::downgrade(&node);
            new_right.as_mut().unwrap().borrow_mut().parent = Some(node_ptr_weak);

            // 子を戻す
            node.borrow_mut().right = new_right;

            (Some(node), old_value)
        }
    }
}

/// nodeの左側に子を追加し，追加された子のポインタを返す
pub fn insert_left<K: Ord, V>(mut node: NodePtr<K, V>, key: K, value: V) -> NodePtr<K, V> {
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
pub fn insert_right<K: Ord, V>(mut node: NodePtr<K, V>, key: K, value: V) -> NodePtr<K, V> {
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
        node::insert::{insert, insert_right},
        print_util::print_as_binary_tree,
    };

    use super::insert_left;

    #[test]
    fn test_insert() {
        let mut root = None;
        print_as_binary_tree(&root);

        (root, _) = insert(root, 5, "first");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 15, "second");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 1, "third");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 3, "forth");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 30, "fifth");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 15, "sixth");
        print_as_binary_tree(&root);
    }

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
}
