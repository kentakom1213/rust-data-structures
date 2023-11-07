use std::fmt::Debug;

// ノードの値が満たすべき条件
pub trait Val
where
    Self: Clone + PartialEq + Debug,
{
}

/// ## Node
/// linkedlistのノード
pub struct Node<T: Val> {
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
}

/// 1つ後のポインタを返す
pub unsafe fn next<T: Val>(ptr: *mut Node<T>) -> Option<*mut Node<T>> {
    unsafe { (*ptr).next }
}

/// 1つ前のポインタを返す
pub unsafe fn prev<T: Val>(ptr: *mut Node<T>) -> Option<*mut Node<T>> {
    unsafe { (*ptr).prev }
}

/// ポインタの後に挿入
pub unsafe fn insert_next<T: Val>(ptr: *mut Node<T>, val: T) {
    let new_ptr = Node::new_pointer(val);

    if let Some(ptr_next) = unsafe { (*ptr).next } {
        unsafe {
            (*new_ptr).next = Some(ptr_next);
            (*ptr_next).prev = Some(new_ptr);
        }
    }

    unsafe {
        (*ptr).next = Some(new_ptr);
        (*new_ptr).prev = Some(ptr);
    };
}

/// ポインタの前に挿入
pub unsafe fn insert_prev<T: Val>(ptr: *mut Node<T>, val: T) {
    let new_ptr = Node::new_pointer(val);

    if let Some(ptr_prev) = unsafe { (*ptr).prev } {
        unsafe {
            (*new_ptr).prev = Some(ptr_prev);
            (*ptr_prev).next = Some(new_ptr);
        }
    }

    unsafe {
        (*ptr).prev = Some(new_ptr);
        (*new_ptr).next = Some(ptr);
    };
}

/// ノードを削除
unsafe fn delete_node<T: Val>(ptr: *mut Node<T>) -> Option<T> {
    if let (Some(ptr_prev), Some(ptr_next)) = unsafe { ((*ptr).prev, (*ptr).next) } {
        unsafe {
            (*ptr_prev).next = Some(ptr_next);
            (*ptr_next).prev = Some(ptr_prev);
        }
    }
    // 生ポインタをBoxに包みなおす
    let node_box = unsafe { Box::from_raw(ptr) };
    Some(node_box.data)
}

impl<T: Val> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({:?})", self.data)
    }
}

/// ## DoubleLinkedList
pub struct DoubleLinkedList<T: Val> {
    size: usize,
    pub head: Option<*mut Node<T>>,
    pub tail: Option<*mut Node<T>>,
}

impl<T: Val> DoubleLinkedList<T> {
    /// 連結リストの作成
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }

    /// 要素数の取得
    pub fn len(&self) -> usize {
        self.size
    }

    /// 先頭に要素を追加
    pub fn insert_head(&mut self, val: T) {
        let new_ptr = Node::new_pointer(val);
        if let Some(head) = self.head {
            unsafe {
                (*head).prev = Some(new_ptr);
                (*new_ptr).next = Some(head);
            }
        }
        self.head = Some(new_ptr);
        if self.size == 0 {
            self.tail = Some(new_ptr);
        }
        self.size += 1;
    }

    /// 末尾に要素を追加
    pub fn insert_tail(&mut self, val: T) {
        let new_ptr = Node::new_pointer(val);
        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(new_ptr);
                (*new_ptr).prev = Some(tail);
            }
        }
        self.tail = Some(new_ptr);
        if self.size == 0 {
            self.head = Some(new_ptr);
        }
        self.size += 1;
    }

    /// 先頭の要素を削除
    pub fn delete_head(&mut self) -> Option<T> {
        if let Some(head) = self.head {
            if let Some(head_next) = unsafe { (*head).next } {
                self.head = Some(head_next);
                unsafe {
                    (*head_next).prev = None;
                }
            }
            // 要素数の修正
            self.size -= 1;
            if self.size == 0 {
                self.head = None;
                self.tail = None;
            }
            // 削除したデータを返す
            let data = unsafe { Box::from_raw(head) }.data;
            Some(data)
        } else {
            None
        }
    }

    /// 末尾の要素を削除
    pub fn delete_tail(&mut self) -> Option<T> {
        if let Some(tail) = self.tail {
            if let Some(tail_prev) = unsafe { (*tail).prev } {
                self.tail = Some(tail_prev);
                unsafe {
                    (*tail_prev).next = None;
                }
            }
            // 要素数の修正
            self.size -= 1;
            if self.size == 0 {
                self.head = None;
                self.tail = None;
            }
            // 削除したデータを返す
            let data = unsafe { Box::from_raw(tail) }.data;
            Some(data)
        } else {
            None
        }
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

    /// 一致する要素があるノードを返す
    pub fn find(&mut self, key: &T) -> Option<*mut Node<T>> {
        let mut ptr = self.head;
        while let Some(rptr) = ptr {
            if unsafe { &(*rptr).data } == key {
                return ptr;
            }
            unsafe {
                ptr = (*rptr).next;
            }
        }
        None
    }

    /// delete
    pub fn delete(&mut self, ptr: *mut Node<T>) -> Option<T> {
        if Some(ptr) == self.head {
            self.delete_head()
        } else if Some(ptr) == self.tail {
            self.delete_tail()
        } else {
            let res = unsafe { delete_node(ptr) };
            if res.is_some() {
                self.size -= 1;
            }
            res
        }
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

// プリミティブ型にimpl
impl Val for isize {}
impl Val for usize {}
impl Val for &str {}
