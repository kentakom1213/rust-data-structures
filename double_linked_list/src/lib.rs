pub mod double_linked_list {

    /// ## Node
    /// linkedlistのノード
    #[derive(Debug)]
    pub struct Node<T> {
        pub prev: Option<*mut Node<T>>,
        pub next: Option<*mut Node<T>>,
        pub data: T,
    }

    impl<T> Node<T> {
        pub fn new(data: T) -> Self {
            Self {
                prev: None,
                next: None,
                data,
            }
        }
    }

    /// 後ろに要素を追加
    pub fn insert_next<T>(this: *mut Node<T>, data: T) {
        // 新しく追加するノード
        let mut new_node = Node::new(data);
        let post_node = unsafe { this.as_mut() }.unwrap().next;
        if let Some(post_node) = post_node {
            unsafe {
                post_node.as_mut().unwrap().prev = Some( &mut new_node as *mut Node<T> );
                *post_node = new_node;
            }
        } else {
            unsafe {
                this.as_mut().unwrap().next = Some( &mut new_node as *mut Node<T> );
            }
        }
    }

    /// 一つ右に進める
    pub fn to_next<T>(this: *mut Node<T>) -> Option<*mut Node<T>> {
        let cur_raw = unsafe { this.as_mut() }.unwrap();
        cur_raw.next
    }

    /// 一つ左に進める
    pub fn to_prev<T>(this: *mut Node<T>) -> Option<*mut Node<T>> {
        let cur_raw = unsafe { this.as_mut() }.unwrap();
        cur_raw.prev
    }
}
