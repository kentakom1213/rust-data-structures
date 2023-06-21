#![allow(dead_code)]

mod linked_list {
    use std::fmt::Debug;

    // ノードの値が満たすべき条件
    pub trait Val
    where
        Self: Clone + PartialEq + Debug,
    {
    }

    /// 連結リストのノード
    pub struct Node<T: Val> {
        pub data: T,
        pub next: Option<*mut Node<T>>,
    }

    impl<T: Val> Node<T> {
        pub fn new(val: T) -> Self {
            Self {
                data: val,
                next: None,
            }
        }

        /// 新しいノードのポインタを作成する
        pub fn new_pointer(val: T) -> *mut Self {
            let new_node = Box::new(Self {
                data: val,
                next: None,
            });
            Box::into_raw(new_node)
        }

        /// 現在のノードの次に値を挿入
        pub fn insert_next(&mut self, val: T) {
            let new_ptr = Self::new_pointer(val);

            if let Some(next_next) = self.next {
                unsafe {
                    (*new_ptr).next = Some(next_next);
                }
            }

            self.next = Some(new_ptr);
        }
    }

    impl<T: Val> Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Node({:?})", self.data)
        }
    }

    /// 連結リスト
    pub struct LinkedList<T: Val> {
        pub head: Option<*mut Node<T>>,
    }

    impl<T: Val> LinkedList<T> {
        /// LinkedListの作成
        pub fn new() -> Self {
            Self {
                head: None,
            }
        }

        /// 要素の追加
        pub fn insert_head(&mut self, val: T) {
            let new_ptr = Node::new_pointer(val);
            if let Some(head) = self.head {
                unsafe {
                    (*new_ptr).next = Some(head);
                }
            }
            self.head = Some(new_ptr);
        }
    }

    impl<T: Val> Debug for LinkedList<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LinkedList([").ok();
            let mut ptr = self.head;
            while let Some(node) = ptr {
                write!(f, "{:?}, ", unsafe { &(*node) }).ok();
                unsafe {
                    ptr = (*node).next;
                }
            }
            // 出力
            write!(f, "])")
        }
    }
}

use linked_list::*;

fn main() {
    impl Val for usize {}

    let mut list = LinkedList::new();
    println!("{:?}", list);

    list.insert_head(1);
    println!("{:?}", list);

    list.insert_head(2);
    println!("{:?}", list);

    list.insert_head(3);
    println!("{:?}", list);

    list.insert_head(4);
    println!("{:?}", list);
}
