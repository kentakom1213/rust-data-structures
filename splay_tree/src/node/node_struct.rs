use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use super::node_pointer::NodePtr;

/// ノードの構造体
pub struct Node<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub parent: Option<Weak<RefCell<Node<K, V>>>>,
    pub left: Option<Rc<RefCell<Node<K, V>>>>,
    pub right: Option<Rc<RefCell<Node<K, V>>>>,
}

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
mod test_node_struct {
    use crate::node::node_struct::Node;

    #[test]
    fn test_create_tree() {
        let root = Node::new(0, "root");

        eprintln!("{:?}", root);
    }
}
