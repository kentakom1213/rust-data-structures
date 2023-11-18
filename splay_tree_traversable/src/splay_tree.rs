//! 親へのポインタも持つスプレー木

use std::fmt::Debug;

/// スプレー木のノード
pub struct SplayTreeNode<T, U> {
    pub key: T,
    pub value: U,
    parent: Option<*mut SplayTreeNode<T, U>>,
    left: Option<*mut SplayTreeNode<T, U>>,
    right: Option<*mut SplayTreeNode<T, U>>,
}

impl<T, U> SplayTreeNode<T, U> {
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
}

// ----- Debug -----
// impl<T, U> Debug for SplayTree<T, U>
// where
//     T: Ord + Debug,
//     U: Debug,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         fmt_inner(f, &self.root, 0);
//         Ok(())
//     }
// }

// /// 再帰的に表示
// #[allow(unused_must_use)]
// fn fmt_inner<T, U>(
//     f: &mut std::fmt::Formatter<'_>,
//     node: &Option<*mut SplayTreeNode<T, U>>,
//     depth: usize,
// ) where
//     T: Ord + Debug,
//     U: Debug,
// {
//     match node {
//         Some(node) => unsafe {
//             fmt_inner(f, &(**node).left, depth + 1);
//             writeln!(
//                 f,
//                 "{}(key:{:?}, value:{:?})",
//                 " ".repeat(depth * 2),
//                 (**node).key,
//                 (**node).value
//             );
//             fmt_inner(f, &(**node).right, depth + 1);
//         },
//         None => {}
//     }
// }

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

/// 再帰的に表示
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
