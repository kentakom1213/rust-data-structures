//! 親へのポインタも持つスプレー木

use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::Debug,
    rc::{Rc, Weak}, borrow::BorrowMut,
};

/// スプレー木のノード
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    parent: Option<Weak<RefCell<Node<K, V>>>>,
    left: Option<Rc<RefCell<Node<K, V>>>>,
    right: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> Node<K, V> {
    /// allocate memory and return pointer
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            parent: None,
            left: None,
            right: None,
        }
    }

    /// rotate
    /// ```not-rust
    ///        pp                     pp
    ///        |                      |
    ///        p                      c    
    ///       / \       (right)      / \   
    ///      c   C  === rotate ==>  A   p  
    ///     / \                        / \
    ///    A   B                      B   C
    /// ```
    fn rotate(&mut self) {
        // if parent doesn't exist
        if self.parent.is_none() {
            return;
        }
        let p = self.parent.take().unwrap();
    }
}

/// スプレー木
pub struct SplayTree<K, V> {
    size: usize,
    pub root: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> SplayTree<K, V> {
    /// initialize tree
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }

    /// insert element and return old value if it exists
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // update root
        let old_node = self
            .root
            .borrow_mut()
            .replace(Rc::new(RefCell::new(Node::new(key, value))));

        // delete old_node's reference
        todo!()
    }

    /// get size of tree
    pub fn len(&self) -> usize {
        self.size
    }

    /// determine if this tree is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

/// splay
fn splay<K: Ord, V>(
    root: Option<Rc<RefCell<Node<K, V>>>>,
    key: &K,
) -> (Option<Rc<RefCell<Node<K, V>>>>, bool) {
    todo!()
}

impl<K, V> SplayTree<K, V>
where
    K: Ord + Debug,
    V: Debug,
{
    pub fn pretty_print(&self) {
        println!("┌─────── Tree ───────");
        fmt_inner(&self.root, 0);
        println!("└────────────────────");
    }
}

/// print recursive
fn fmt_inner<K, V>(node: &Option<Rc<RefCell<Node<K, V>>>>, depth: usize)
where
    K: Ord + Debug,
    V: Debug,
{
    match node.as_ref() {
        Some(node) => {
            fmt_inner(&node.borrow().left, depth + 1);
            println!(
                "│{}({:?}, {:?})",
                " ".repeat(depth * 2),
                node.borrow().key,
                node.borrow().value
            );
            fmt_inner(&node.borrow().right, depth + 1);
        }
        None => {}
    }
}
