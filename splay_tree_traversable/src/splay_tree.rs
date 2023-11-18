//! 親へのポインタも持つスプレー木

use std::{cmp::Ordering, fmt::Debug};

/// スプレー木のノード
pub struct SplayTreeNode<T, U> {
    pub key: T,
    pub value: U,
    parent: Option<*mut SplayTreeNode<T, U>>,
    left: Option<*mut SplayTreeNode<T, U>>,
    right: Option<*mut SplayTreeNode<T, U>>,
}

impl<T, U> SplayTreeNode<T, U> {
    /// allocate memory and return pointer
    fn create(key: T, value: U) -> *mut Self {
        let node = Box::new(Self {
            key,
            value,
            parent: None,
            left: None,
            right: None,
        });
        Box::into_raw(node)
    }

    /// free memory and return key, value
    fn destroy(node_pointer: *mut Self) -> (T, U) {
        let Self { key, value, .. } = unsafe { *Box::from_raw(node_pointer) };
        (key, value)
    }
}

/// スプレー木
pub struct SplayTree<T, U> {
    size: usize,
    pub root: Option<*mut SplayTreeNode<T, U>>,
}

impl<T, U> SplayTree<T, U> {
    /// initialize tree
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }

    /// insert element and return old value if it exists
    pub fn insert(&mut self, key: T, value: U) -> Option<U> {
        // get old value
        let old_value = self.root.take().map(|node| SplayTreeNode::destroy(node).1);
        // update root
        self.root = Some(SplayTreeNode::create(key, value));

        old_value
    }

    /// get size of tree
    pub fn len(&self) -> usize {
        self.size
    }

    /// determine if this tree is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

//     /// get child of parent
//     unsafe fn parent_child<T: Ord, U>(
//         &mut self,
//         parent: *mut SplayTreeNode<T, U>,
//     ) -> *mut SplayTreeNode<T, U> {
//         if (*parent).parent.is_none() {
//             self.root
//         }
//     }
}

/// rotate right
/// ```not-rust
///        Y                      X    
///       / \       right        / \   
///      X   C  === rotate ==>  A   Y  
///     / \                        / \
///    A   B                      B   C
/// ```
unsafe fn rotate_right<T: Ord, U>(
    root: Option<*mut SplayTreeNode<T, U>>,
) -> Option<*mut SplayTreeNode<T, U>> {
    todo!()
}

/// rotate left
/// ```not-rust
///      X                          Y  
///     / \         left           / \
///    A   Y    === rotate ==>    X   C
///       / \                    / \   
///      B   C                  A   B  
/// ```
unsafe fn rotate_left<T: Ord, U>(
    root: Option<*mut SplayTreeNode<T, U>>,
) -> Option<*mut SplayTreeNode<T, U>> {
    todo!()
}

/// splay
unsafe fn splay<T: Ord, U>(
    root: Option<*mut SplayTreeNode<T, U>>,
    key: &T,
) -> (Option<*mut SplayTreeNode<T, U>>, bool) {
    todo!()
}

impl<T, U> SplayTree<T, U>
where
    T: Ord + Debug,
    U: Debug,
{
    pub fn pretty_print(&self) {
        println!("┌─────── Tree ───────");
        fmt_inner(&self.root, 0);
        println!("└────────────────────");
    }
}

/// print recursive
fn fmt_inner<T, U>(node: &Option<*mut SplayTreeNode<T, U>>, depth: usize)
where
    T: Ord + Debug,
    U: Debug,
{
    match node {
        Some(node) => unsafe {
            fmt_inner(&(**node).left, depth + 1);
            println!(
                "│{}({:?}, {:?})",
                " ".repeat(depth * 2),
                (**node).key,
                (**node).value
            );
            fmt_inner(&(**node).right, depth + 1);
        },
        None => {}
    }
}
