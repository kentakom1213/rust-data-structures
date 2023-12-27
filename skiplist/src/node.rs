//! ノード

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type Node<T> = Option<Rc<RefCell<ListNode<T>>>>;
pub type NodeParent<T> = Option<Weak<RefCell<ListNode<T>>>>;

/// シンプルな双方向連結リスト
#[derive(Debug)]
pub struct ListNode<T> {
    pub data: T,
    pub next: Node<T>,
    pub prev: NodeParent<T>,
}

impl<T> ListNode<T> {
    /// Option<Rc<RefCell>>に包んだノードを作成する
    pub fn new(data: T) -> Rc<RefCell<ListNode<T>>> {
        Rc::new(RefCell::new(ListNode {
            data,
            next: None,
            prev: None,
        }))
    }
}

/// 一つ後ろに進む
pub fn next<T>(node: Node<T>) -> Node<T> {
    node.and_then(|x| x.borrow().next.clone())
}

/// 後ろに要素を追加
pub fn insert_next<T>(mut node: Node<T>, data: T) -> Node<T> {
    let tmp = ListNode::new(data);
    // 新しいノードの前に自分を追加
    tmp.borrow_mut()
        .prev
        .replace(Rc::downgrade(&node.as_ref().unwrap().clone()));
    // 新しいノードの後ろに自分の後ろのノードを追加
    let nxt = node.as_mut().unwrap().borrow_mut().next.take();
    if let Some(nxt) = nxt {
        tmp.borrow_mut().next.replace(nxt);
    }
    // 自分の後ろに新しいノードを追加
    node.as_mut().unwrap().borrow_mut().next.replace(tmp);
    node
}
