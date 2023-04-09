use std::{cmp::Ordering, fmt, mem, ops::DerefMut};

#[derive(Debug)]
pub struct BinaryTreeNode<T> {
    pub value: T,
    pub left: Option<Box<BinaryTreeNode<T>>>,
    pub right: Option<Box<BinaryTreeNode<T>>>,
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: Option<Box<BinaryTreeNode<T>>>,
}

impl<T: Ord + fmt::Debug> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn search(&mut self, value: &T) -> bool {
        // 身代わりを用意する
        search_inner(value, &self.root)
    }

    pub fn insert(&mut self, value: T) -> bool {
        let res = insert_inner(value, &mut self.root);
        res
    }
}

impl<T: Ord + fmt::Debug> BinaryTree<T> {
    pub fn pretty_print(&self) {
        pretty_print_inner(&self.root, 0);
    }
}

/// 整形して表示
fn pretty_print_inner<K: Ord + fmt::Debug>(node: &Option<Box<BinaryTreeNode<K>>>, depth: usize) {
    match node {
        Some(ref node) => {
            pretty_print_inner(&node.left, depth + 2);
            println!("{}{:?}", " ".repeat(depth * 2), node.value);
            pretty_print_inner(&node.right, depth + 2);
        }
        None => {}
    }
}

/// keyを検索する
fn search_inner<T: Ord>(value: &T, root: &Option<Box<BinaryTreeNode<T>>>) -> bool {
    if root.is_none() {
        return false;
    }
    let node = root.as_ref().unwrap();
    match value.cmp(&node.value) {
        Ordering::Equal => true,
        Ordering::Less => search_inner(value, &node.left),
        Ordering::Greater => search_inner(value, &node.right),
    }
}

/// keyを挿入するべき位置にあるノードを返す
fn insert_inner<T: Ord>(value: T, root: &mut Option<Box<BinaryTreeNode<T>>>) -> bool {
    if root.is_none() {
        let mut new_node = Some(Box::new(
            BinaryTreeNode {
                value,
                left: None,
                right: None,
            }
        ));
        std::mem::swap(root, &mut new_node);
        return true;
    }
    let node = root.as_mut().unwrap();
    match value.cmp(&node.value) {
        Ordering::Equal => false,
        Ordering::Less => insert_inner(value, &mut node.left),
        Ordering::Greater => insert_inner(value, &mut node.right),
    }
}
