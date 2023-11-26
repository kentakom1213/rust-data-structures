//! Rc<RefCell>を使った安全な双方向リスト

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    next: RefCell<Option<Rc<Node<T>>>>,
    prev: RefCell<Weak<Node<T>>>,
}

impl<T> Node<T> {
    /// ノードの初期化
    pub fn new(x: T) -> Self {
        Self {
            data: x,
            next: RefCell::new(None),
            prev: RefCell::new(Weak::new()),
        }
    }
}

#[derive(Debug)]
pub struct DoubleLinkedList<T> {
    head: RefCell<Option<Rc<Node<T>>>>,
    tail: RefCell<Weak<Node<T>>>,
    size: usize,
}

impl<T> DoubleLinkedList<T> {
    /// 双方向連結リストを初期化する
    pub fn new() -> Self {
        Self {
            head: RefCell::new(None),
            tail: RefCell::new(Weak::new()),
            size: 0,
        }
    }

    /// 最初の要素を追加する
    fn insert_first_element(&mut self, x: T) {
        let node = Rc::new(Node::new(x));
        *self.head.borrow_mut() = Some(node.clone());
        *self.tail.borrow_mut() = Rc::downgrade(&node);
        self.size += 1;
    }

    /// 末尾に要素`x`を追加する
    pub fn push_back(&mut self, x: T) {
        if self.is_empty() {
            self.insert_first_element(x);
            return;
        }
        let node = Rc::new(Node::new(x));
        *self.tail.borrow_mut().upgrade().unwrap().next.borrow_mut() = Some(node.clone());
        *self.tail.borrow_mut() = Rc::downgrade(&node);
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dll() {
        let mut dll: DoubleLinkedList<i8> = DoubleLinkedList::new();

        dll.push_back(1);
        println!("{:#?}", dll);

        dll.push_back(2);
        println!("{:#?}", dll);
    }
}
