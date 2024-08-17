//! ノードの構造体

use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::Debug,
    mem,
    rc::{Rc, Weak},
};

/// ノードの構造体
pub struct Node<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub parent: Option<Weak<RefCell<Node<K, V>>>>,
    pub left: Option<Rc<RefCell<Node<K, V>>>>,
    pub right: Option<Rc<RefCell<Node<K, V>>>>,
}

/// ノードのポインタ
pub type NodePtr<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

/// 親ノードのポインタ
pub type ParentPtr<K, V> = Option<Weak<RefCell<Node<K, V>>>>;

impl<K: Ord, V> Node<K, V> {
    /// 葉ノードを作成する
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            parent: None,
            left: None,
            right: None,
        }
    }

    /// ノードのポインタを確保する
    pub fn node_ptr(key: K, value: V) -> NodePtr<K, V> {
        Some(Rc::new(RefCell::new(Self::new(key, value))))
    }
}

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
fn insert<K: Ord, V: Clone>(node: NodePtr<K, V>, key: K, value: V) -> (NodePtr<K, V>, Option<V>) {
    let Some(node) = node else {
        return (Node::node_ptr(key, value), None);
    };

    // キーの比較
    let comp = key.cmp(&node.as_ref().borrow().key);

    match comp {
        Ordering::Less => {
            // 左の子に挿入
            let left = node.as_ref().borrow_mut().left.take();
            let (mut new_left, old_value) = insert(left, key, value);

            // new_leftの親の更新
            let node_ptr_weak = Rc::downgrade(&node);
            new_left.as_mut().unwrap().as_ref().borrow_mut().parent = Some(node_ptr_weak);

            // 子を戻す
            node.as_ref().borrow_mut().left = new_left;

            (Some(node), old_value)
        }
        Ordering::Equal => {
            // valueを置き換える
            let old_value = mem::replace(&mut node.as_ref().borrow_mut().value, value);

            (Some(node), Some(old_value))
        }
        Ordering::Greater => {
            // 左の子に挿入
            let right = node.as_ref().borrow_mut().right.take();
            let (mut new_right, old_value) = insert(right, key, value);

            // new_rightの親の更新
            let node_ptr_weak = Rc::downgrade(&node);
            new_right.as_mut().unwrap().as_ref().borrow_mut().parent = Some(node_ptr_weak);

            // 子を戻す
            node.as_ref().borrow_mut().right = new_right;

            (Some(node), old_value)
        }
    }
}

/// rootを根とする木で，xに一致するキーをもつノードの参照を返す
pub fn find<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePtr<K, V>) {
    let mut node = root.clone();
    while let Some(inner) = node.clone() {
        let comp = x.cmp(&inner.borrow().key);
        match comp {
            Ordering::Less => node = inner.borrow().left.clone(),
            Ordering::Equal => break,
            Ordering::Greater => node = inner.borrow().right.clone(),
        }
    }
    (root, node)
}

/// nodeを1つ上に持ってくるように回転する
fn rotate<K: Ord, V>(node: NodePtr<K, V>) -> NodePtr<K, V> {
    let mut node = node?;
    // 親
    let Some(par) = node.as_ref().borrow_mut().parent.take() else {
        return Some(node);
    };

    todo!()
}

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

impl NodeState {
    /// 与えられたノードが
    /// - 空のノード
    /// - 根ノード
    /// - 親の左の子
    /// - 親の右の子
    ///
    /// のどれかを判定する．
    fn get<K: Ord, V>(node: &NodePtr<K, V>) -> Self {
        let Some(inner) = node else {
            return NodeState::Nil;
        };
        let Some(par) = inner.borrow().parent.clone() else {
            return NodeState::Root;
        };
        // 左の子である場合
        let par = par.upgrade().unwrap();
        if Rc::ptr_eq(par.borrow().left.as_ref().unwrap(), &inner) {
            NodeState::LeftChild
        } else {
            NodeState::RightChild
        }
    }
}

impl<K: Ord + Debug, V: Debug> Debug for Node<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.left, &self.right) {
            (None, None) => f
                .debug_struct("Node")
                .field(&"key", &self.key)
                .field(&"value", &self.value)
                .finish(),
            (Some(_), None) => f
                .debug_struct("Node")
                .field(&"key", &self.key)
                .field(&"value", &self.value)
                .field(&"left", &self.left)
                .finish(),
            (None, Some(_)) => f
                .debug_struct("Node")
                .field(&"key", &self.key)
                .field(&"value", &self.value)
                .field(&"right", &self.right)
                .finish(),
            (Some(_), Some(_)) => f
                .debug_struct("Node")
                .field(&"key", &self.key)
                .field(&"value", &self.value)
                .field(&"left", &self.left)
                .field(&"right", &self.right)
                .finish(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        node::{insert, NodeState},
        print_util::print_as_binary_tree,
    };

    use super::{find, Node};

    #[test]
    fn test_create_tree() {
        let root = Node::new(0, "root");

        eprintln!("{:?}", root);
    }

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
    fn test_find() {
        let mut root = None;
        (root, _) = insert(root, 5, "first");
        (root, _) = insert(root, 15, "second");
        (root, _) = insert(root, 1, "third");
        (root, _) = insert(root, 3, "forth");
        (root, _) = insert(root, 30, "fifth");

        let find_5;
        (root, find_5) = find(root, &5);
        print_as_binary_tree(&root);
        println!("{:?}", &find_5);

        let find_20;
        (root, find_20) = find(root, &2);
        print_as_binary_tree(&root);
        println!("{:?}", &find_20);

        let find_15;
        print_as_binary_tree(&root);
        (root, find_15) = find(root, &15);
        println!("{:?}", &find_15);

        (root, _) = insert(root, 20, "sixth");
        print_as_binary_tree(&root);
        println!("{:?}", &find_15);
    }

    #[test]
    fn test_nodestate() {
        let mut root = None;
        (root, _) = insert(root, 5, "first");
        (root, _) = insert(root, 15, "second");
        (root, _) = insert(root, 1, "third");
        (root, _) = insert(root, 3, "forth");
        (root, _) = insert(root, 30, "fifth");

        print_as_binary_tree(&root);

        let find_5;
        (root, find_5) = find(root, &5);
        println!("find_5 = {:?}", NodeState::get(&find_5));

        let find_20;
        (root, find_20) = find(root, &20);
        println!("find_20 = {:?}", NodeState::get(&find_20));

        let find_15;
        (root, find_15) = find(root, &15);
        println!("find_15 = {:?}", NodeState::get(&find_15));

        (root, _) = insert(root, 20, "sixth");
        print_as_binary_tree(&root);

        let find_20;
        (root, find_20) = find(root, &20);
        println!("find_20 = {:?}", NodeState::get(&find_20));
    }
}
