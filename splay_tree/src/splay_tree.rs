use std::mem::{replace, swap};
use std::ops::DerefMut;
use std::{cmp::Ordering, fmt::Debug};

/// # Node
pub struct Node<T, U>
where
    T: Ord,
{
    pub key: T,
    pub value: U,
    pub left: Option<Box<Node<T, U>>>,
    pub right: Option<Box<Node<T, U>>>,
}

impl<T: Ord, U> Node<T, U> {
    pub fn new(key: T, value: U) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }
}

/// # SplayTree
/// スプレー木のクラス
pub struct SplayTree<T, U>
where
    T: Ord,
{
    size: usize,
    root: Option<Box<Node<T, U>>>,
}

impl<T, U> SplayTree<T, U>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    /// ## search
    /// - 値の検索を行う
    pub fn search(&mut self, key: &T) -> Option<&U> {
        search_inner(&self.root, key)
    }

    /// ## insert
    /// - 値の挿入
    pub fn insert(&mut self, key: T, value: U) -> bool {
        let res = search_mut(&mut self.root, &key);
        if res.is_none() {
            *res = Some(Box::new(Node::new(key, value)));
            self.size += 1; // 要素数をインクリメント
            true
        } else {
            false
        }
    }
}

/// keyを検索する
fn search_inner<'a, T: Ord, U>(root: &'a Option<Box<Node<T, U>>>, value: &T) -> Option<&'a U> {
    if root.is_none() {
        return None;
    }
    let node = root.as_ref().unwrap();
    match value.cmp(&node.key) {
        Ordering::Equal => Some(&node.value),
        Ordering::Less => search_inner(&node.left, value),
        Ordering::Greater => search_inner(&node.right, value),
    }
}

/// keyを挿入するべき位置にあるノードを返す
fn search_mut<'a, T: Ord, U>(
    root: &'a mut Option<Box<Node<T, U>>>,
    key: &T,
) -> &'a mut Option<Box<Node<T, U>>> {
    if root.is_none() {
        return root;
    }
    match key.cmp(&root.as_ref().unwrap().key) {
        Ordering::Equal => root,
        Ordering::Less => search_mut(&mut root.as_deref_mut().unwrap().left, key),
        Ordering::Greater => search_mut(&mut root.as_deref_mut().unwrap().right, key),
    }
}

// ----- Debug -----
impl<T, U> SplayTree<T, U>
where
    T: Ord + Debug,
    U: Debug,
{
    /// 整形して表示する
    pub fn pretty_print(&self) {
        pretty_print_inner(&self.root, 0);
    }
}

/// 再帰的に表示
fn pretty_print_inner<T, U>(node: &Option<Box<Node<T, U>>>, depth: usize)
where
    T: Ord + Debug,
    U: Debug,
{
    match node {
        Some(ref node) => {
            pretty_print_inner(&node.left, depth + 2);
            println!(
                "{}(key: {:?}, value: {:?})",
                " ".repeat(depth * 2),
                node.key,
                node.value
            );
            pretty_print_inner(&node.right, depth + 2);
        }
        None => {}
    }
}
