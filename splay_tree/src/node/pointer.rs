//! ノードのポインタ

macro_rules! generate_getters {
    // 不変参照用のgetterを生成
    ($name:ident, $field:ident, $return_type:ty) => {
        fn $name(&self) -> $return_type {
            let node_ref = self.borrow();
            std::cell::Ref::map(node_ref, |node| &node.$field)
        }
    };

    // 可変参照用のgetterを生成
    ($name:ident, $field:ident, $return_type:ty, mut) => {
        fn $name(&mut self) -> $return_type {
            let node_mut = self.borrow_mut();
            std::cell::RefMut::map(node_mut, |node| &mut node.$field)
        }
    };
}

use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::{Rc, Weak},
};

use crate::node::state::NodeState;

/// ノードの構造体
pub struct Node<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub parent: Option<ParentPtr<K, V>>,
    pub left: Option<NodePtr<K, V>>,
    pub right: Option<NodePtr<K, V>>,
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
        Rc::new(RefCell::new(Self::new(key, value)))
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

/// ノードのポインタ
pub type NodePtr<K, V> = Rc<RefCell<Node<K, V>>>;

/// 親ノードのポインタ
pub type ParentPtr<K, V> = Weak<RefCell<Node<K, V>>>;

/// ポインタに対する操作
pub trait NodeOps<K: Ord, V> {
    /// 与えられたノードが子ノードであるかを判定する
    fn is_child(&self) -> bool;
    /// 与えられたノードが
    /// - 空のノード
    /// - 根ノード
    /// - 親の左の子
    /// - 親の右の子
    ///
    /// のどれかを判定する．
    fn get_state(&self) -> NodeState;

    /// ポインタの同一性判定
    fn is_same(&self, other: &Self) -> bool;
    /// ポインタの半順序
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>;

    /// 親のポインタを取得する
    fn get_parent_ptr(&self) -> Option<NodePtr<K, V>>;

    /// 親へのポインタを切り離す
    fn take_parent(&mut self) -> Option<ParentPtr<K, V>>;
    /// 親へのポインタを切り離し，強参照を取得する
    fn take_parent_strong(&mut self) -> Option<NodePtr<K, V>>;
    /// 左の子へのポインタを切り離す
    fn take_left(&mut self) -> Option<NodePtr<K, V>>;
    /// 右の子へのポインタを切り離す
    fn take_right(&mut self) -> Option<NodePtr<K, V>>;

    /// 親の参照を取得する
    fn parent(&self) -> Ref<Option<ParentPtr<K, V>>>;
    /// 親の可変参照を取得する
    fn parent_mut(&mut self) -> RefMut<Option<ParentPtr<K, V>>>;
    /// 左の子への参照を取得する
    fn left(&self) -> Ref<Option<NodePtr<K, V>>>;
    /// 左の子への可変参照を取得する
    fn left_mut(&mut self) -> RefMut<Option<NodePtr<K, V>>>;
    /// 右の子への参照を取得する
    fn right(&self) -> Ref<Option<NodePtr<K, V>>>;
    /// 右の子への可変参照を取得する
    fn right_mut(&mut self) -> RefMut<Option<NodePtr<K, V>>>;

    /// キーへの参照を取得する
    fn key(&self) -> Ref<K>;
    /// バリューへの参照を取得する
    fn value(&self) -> Ref<V>;
    /// バリューへの可変参照を取得する
    fn value_mut(&mut self) -> RefMut<V>;

    /// 親ポインタに変換する
    fn to_weak_ptr(&self) -> ParentPtr<K, V>;
}

impl<K: Ord, V> NodeOps<K, V> for NodePtr<K, V> {
    fn is_child(&self) -> bool {
        self.parent().is_some()
    }

    fn get_state(&self) -> NodeState {
        let par = self.get_parent_ptr();

        if par.is_none() {
            return NodeState::Root;
        }

        if par
            .clone()
            .is_some_and(|ptr| ptr.left().as_ref().is_some_and(|left| left.is_same(self)))
        {
            return NodeState::LeftChild;
        }

        if par.is_some_and(|ptr| {
            ptr.right()
                .as_ref()
                .is_some_and(|right| right.is_same(self))
        }) {
            return NodeState::RightChild;
        }

        unreachable!()
    }

    fn is_same(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.borrow().key.partial_cmp(&other.borrow().key)
    }

    fn get_parent_ptr(&self) -> Option<NodePtr<K, V>> {
        self.parent().as_ref().map(|p| p.to_strong_ptr())
    }

    fn take_parent(&mut self) -> Option<ParentPtr<K, V>> {
        self.as_ref().borrow_mut().parent.take()
    }

    fn take_parent_strong(&mut self) -> Option<NodePtr<K, V>> {
        self.as_ref()
            .borrow_mut()
            .parent
            .take()
            .map(|p| p.upgrade().unwrap())
    }

    fn take_left(&mut self) -> Option<NodePtr<K, V>> {
        let mut left = self.as_ref().borrow_mut().left.take();
        if let Some(left_inner) = left.as_mut() {
            *left_inner.parent_mut() = None;
        }
        left
    }

    fn take_right(&mut self) -> Option<NodePtr<K, V>> {
        let mut right = self.as_ref().borrow_mut().right.take();
        if let Some(right_inner) = right.as_mut() {
            *right_inner.parent_mut() = None;
        }
        right
    }

    fn to_weak_ptr(&self) -> ParentPtr<K, V> {
        Rc::downgrade(self)
    }

    // 不変参照用のgetterを生成
    generate_getters!(parent, parent, Ref<Option<ParentPtr<K, V>>>);
    generate_getters!(left, left, Ref<Option<NodePtr<K, V>>>);
    generate_getters!(right, right, Ref<Option<NodePtr<K, V>>>);
    generate_getters!(key, key, Ref<K>);
    generate_getters!(value, value, Ref<V>);

    // 可変参照用のgetterを生成
    generate_getters!(parent_mut, parent, RefMut<Option<ParentPtr<K, V>>>, mut);
    generate_getters!(left_mut, left, RefMut<Option<NodePtr<K, V>>>, mut);
    generate_getters!(right_mut, right, RefMut<Option<NodePtr<K, V>>>, mut);
    generate_getters!(value_mut, value, RefMut<V>, mut);
}

/// 弱参照の操作
pub trait ParentOps<K: Ord, V> {
    /// NodePtrへの変換
    fn to_strong_ptr(&self) -> NodePtr<K, V>;
}

impl<K: Ord, V> ParentOps<K, V> for ParentPtr<K, V> {
    fn to_strong_ptr(&self) -> NodePtr<K, V> {
        self.upgrade()
            .unwrap_or_else(|| panic!("Failed to upgrade weak reference"))
    }
}

#[cfg(test)]
mod test_pointer {
    use crate::node::pointer::{Node, NodeOps};

    #[test]
    fn test_create_tree() {
        let root = Node::new(0, "root");

        eprintln!("{:?}", root);
    }

    #[test]
    fn test_ref() {
        let mut node = Node::node_ptr(1, "first");

        // 不変参照
        {
            let node_ref = node.key();
            println!("node_ref = {node_ref:?}");
            assert_eq!(*node_ref, 1);

            let val_ref = node.value();
            println!("val_ref = {val_ref:?}");
            assert_eq!(*val_ref, "first");
        }

        // 可変参照
        {
            let mut val_mut = node.value_mut();
            println!("val_mut = {val_mut:?}");

            *val_mut = "changed";
        }

        println!("node = {node:?}");

        // 不変参照
        {
            let node_ref = node.key();
            println!("node_ref = {node_ref:?}");
            assert_eq!(*node_ref, 1);

            let val_ref = node.value();
            println!("val_ref = {val_ref:?}");
            assert_eq!(*val_ref, "changed");
        }
    }
}
