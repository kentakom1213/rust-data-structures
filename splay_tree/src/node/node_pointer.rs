//! ノードのポインタ

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};

use super::{node_struct::Node, state::NodeState};

/// ノードのポインタ
pub type NodePtr<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

/// 親ノードのポインタ
pub type ParentPtr<K, V> = Option<Weak<RefCell<Node<K, V>>>>;

/// ポインタに対する操作
pub trait NodeOps<K: Ord, V> {
    /// 与えられたノードが
    /// - 空のノード
    /// - 根ノード
    /// - 親の左の子
    /// - 親の右の子
    ///
    /// のどれかを判定する．
    fn get_state(&self) -> NodeState;
    /// キーへの参照を取得する
    fn get_key(&self) -> Option<Ref<K>>;
    /// バリューへの参照を取得する
    fn get_value(&self) -> Option<Ref<V>>;
    // /// バリューへの可変参照を取得する
    fn get_value_mut(&mut self) -> Option<RefMut<V>>;
}

impl<K: Ord, V> NodeOps<K, V> for NodePtr<K, V> {
    fn get_state(&self) -> NodeState {
        let Some(inner) = self else {
            return NodeState::Nil;
        };
        let Some(par) = inner.borrow().parent.clone() else {
            return NodeState::Root;
        };
        // 左の子である場合
        let par = par.upgrade().unwrap();
        if par
            .borrow()
            .left
            .as_ref()
            .is_some_and(|left| Rc::ptr_eq(left, inner))
        {
            NodeState::LeftChild
        } else {
            NodeState::RightChild
        }
    }
    fn get_key(&self) -> Option<Ref<K>> {
        let key_ref = self.as_ref()?.borrow();
        Some(Ref::map(key_ref, |node| &node.key))
    }
    fn get_value(&self) -> Option<Ref<V>> {
        let value_ref = self.as_ref()?.borrow();
        Some(Ref::map(value_ref, |node| &node.value))
    }
    fn get_value_mut(&mut self) -> Option<RefMut<V>> {
        let value_ref = self.as_ref()?.borrow_mut();
        Some(RefMut::map(value_ref, |node| &mut node.value))
    }
}

#[cfg(test)]
mod test_pointer {

    use crate::node::node_struct::Node;

    use super::NodeOps;

    #[test]
    fn test_ref() {
        let mut node = Node::node_ptr(1, "first");

        // 不変参照
        {
            let key_ref = node.get_key();
            println!("key_ref = {key_ref:?}");
            assert_eq!(*key_ref.unwrap(), 1);

            let val_ref = node.get_value();
            println!("val_ref = {val_ref:?}");
            assert_eq!(*val_ref.unwrap(), "first");
        }

        // 可変参照
        {
            let val_mut = node.get_value_mut();
            println!("val_mut = {val_mut:?}");

            *val_mut.unwrap() = "changed";
        }

        println!("node = {node:?}");

        // 不変参照
        {
            let key_ref = node.get_key();
            println!("key_ref = {key_ref:?}");
            assert_eq!(*key_ref.unwrap(), 1);

            let val_ref = node.get_value();
            println!("val_ref = {val_ref:?}");
            assert_eq!(*val_ref.unwrap(), "changed");
        }
    }
}
