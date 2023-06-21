pub mod double_linked_list {
    use std::fmt::Debug;

    // ノードの値が満たすべき条件
    pub trait Val
    where
        Self: Clone + PartialEq + Debug,
    {
    }

    /// ## Node
    /// linkedlistのノード
    pub struct Node<T> {
        pub data: T,
        pub prev: Option<*mut Node<T>>,
        pub next: Option<*mut Node<T>>,
    }

    impl<T: Val> Node<T> {
        /// ノードの追加
        pub fn new(val: T) -> Self {
            Self {
                data: val,
                prev: None,
                next: None,
            }
        }

        /// 新しいノードのポインタを作成する
        pub fn new_pointer(val: T) -> *mut Self {
            let new_node = Box::new(Self {
                data: val,
                prev: None,
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

    /// ## DoubleLinkedList
    pub struct DoubleLinkedList<T: Val> {
        pub head: Option<*mut Node<T>>,
        pub tail: Option<*mut Node<T>>,
    }

    impl<T: Val> DoubleLinkedList<T> {
        /// 連結リストの作成
        pub fn new() -> Self {
            Self {
                head: None,
                tail: None,
            }
        }

        /// 先頭に要素を追加
        pub fn insert_head(&mut self, val: T) {
            let new_ptr = Node::new_pointer(val);
            if let Some(head) = self.head {
                unsafe {
                    (*new_ptr).next = Some(head);
                }
            }
            self.head = Some(new_ptr);
        }

        /// i番目のノードの取得
        pub fn nth(&mut self, n: usize) -> Option<*mut Node<T>> {
            let mut ptr = self.head;
            for _ in 0..n {
                if let Some(rptr) = ptr {
                    unsafe {
                        ptr = (*rptr).next;
                    }
                } else {
                    return None;
                }
            }
            ptr
        }
    }

    impl<T: Val> Debug for DoubleLinkedList<T> {
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

use double_linked_list::*;

fn main() {
    impl Val for usize {}

    let mut list = DoubleLinkedList::new();

    for i in 0..10 {
        list.insert_head(i);
    }

    println!("{:?}", &list);

    // 連番で取得
    let mut ptr = list.head;

    while let Some(rptr) = ptr {
        println!("{:?}", ptr);
        ptr = unsafe { rptr.as_mut() }.unwrap().next;
    }
}
