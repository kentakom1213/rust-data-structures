#![allow(unused_must_use)]

use std::mem::{replace, swap};
use std::{cmp::Ordering, fmt::Debug};

/// # Node
#[derive(Debug)]
pub struct Node<T: Ord, U> {
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
pub struct SplayTree<T: Ord, U> {
    size: usize,
    pub root: Option<Box<Node<T, U>>>,
}

impl<T, U> SplayTree<T, U>
where
    T: Ord + Clone,
    U: Debug,
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
    /// 値の検索を行う
    pub fn get(&mut self, key: &T) -> Option<&U> {
        if self.splay(key) {
            Some(&self.root.as_deref().unwrap().value)
        } else {
            None
        }
    }

    /// ## insert
    /// 値の挿入
    pub fn insert(&mut self, key: T, value: U) -> bool {
        // rootの取り出し
        let root = replace(&mut self.root, None);
        // splay操作
        let (mut tmp_root, is_found) = splay_inner(root, &key);
        if is_found {
            self.root = tmp_root;
            return false;
        }
        // 挿入
        self.root = Some(Box::new(Node::new(key.clone(), value)));
        if tmp_root.is_some() {
            match key.cmp(&tmp_root.as_deref().unwrap().key) {
                Ordering::Equal => unreachable!(),
                Ordering::Less => {
                    let mut new_left = replace(&mut tmp_root.as_deref_mut().unwrap().left, None);
                    swap(&mut self.root.as_deref_mut().unwrap().left, &mut new_left);
                    swap(&mut self.root.as_deref_mut().unwrap().right, &mut tmp_root);
                }
                Ordering::Greater => {
                    let mut new_right = replace(&mut tmp_root.as_deref_mut().unwrap().right, None);
                    swap(&mut self.root.as_deref_mut().unwrap().right, &mut new_right);
                    swap(&mut self.root.as_deref_mut().unwrap().left, &mut tmp_root);
                }
            }
        }
        true
    }

    /// ## splay
    /// スプレー操作をおこなう
    /// - 戻り値
    ///   - `bool`：要素が存在したかどうか
    pub fn splay(&mut self, key: &T) -> bool {
        // 根の取り出し
        let root = replace(&mut self.root, None);
        // スプレー操作
        let (new_root, is_found) = splay_inner(root, key);
        self.root = new_root;
        is_found
    }
}

/// splay操作を行う
/// - 戻り値
///   - `Option<Box<Node<T, U>>>`：新しく根となるノード
///   - `bool`：目的の値が存在したかどうか
fn splay_inner<T: Ord, U>(
    mut root: Option<Box<Node<T, U>>>,
    key: &T,
) -> (Option<Box<Node<T, U>>>, bool) {
    if root.is_none() {
        return (root, false);
    }
    // 孫 → 子
    match key.cmp(&root.as_deref().unwrap().key) {
        Ordering::Equal => (root, true),
        Ordering::Less => {
            // 左の子
            let left = &mut root.as_deref_mut().unwrap().left;
            if left.is_none() {
                return (root, false);
            }
            match key.cmp(&left.as_deref().unwrap().key) {
                Ordering::Equal => {
                    // 左の子をrootに
                    (rotate_right(root), true)
                }
                Ordering::Less => {
                    // 孫をsplay
                    let left_left = replace(&mut left.as_deref_mut().unwrap().left, None);
                    let (mut new_left_left, is_found) = splay_inner(left_left, key);
                    swap(&mut left.as_deref_mut().unwrap().left, &mut new_left_left);
                    // 親を右に回転
                    let tmp_child = rotate_right(root);
                    // さらに右に回転
                    (rotate_right(tmp_child), is_found)
                }
                Ordering::Greater => {
                    // 孫をsplay
                    let left_right = replace(&mut left.as_deref_mut().unwrap().right, None);
                    let (mut new_left_right, is_found) = splay_inner(left_right, key);
                    swap(&mut left.as_deref_mut().unwrap().right, &mut new_left_right);
                    // 左の子を左に回転
                    let left = replace(&mut root.as_deref_mut().unwrap().left, None);
                    let mut new_left = rotate_left(left);
                    swap(&mut root.as_deref_mut().unwrap().left, &mut new_left);
                    // さらに右に回転
                    (rotate_right(root), is_found)
                }
            }
        }
        Ordering::Greater => {
            // 右の子
            let right = &mut root.as_deref_mut().unwrap().right;
            if right.is_none() {
                return (root, false);
            }
            match key.cmp(&right.as_deref().unwrap().key) {
                Ordering::Equal => {
                    // 右の子をrootに
                    (rotate_left(root), true)
                }
                Ordering::Less => {
                    // 孫をsplay
                    let right_left = replace(&mut right.as_deref_mut().unwrap().left, None);
                    let (mut new_right_left, is_found) = splay_inner(right_left, key);
                    swap(&mut right.as_deref_mut().unwrap().left, &mut new_right_left);
                    // 右の子を右に回転
                    let right = replace(&mut root.as_deref_mut().unwrap().right, None);
                    let mut new_right = rotate_right(right);
                    swap(&mut root.as_deref_mut().unwrap().right, &mut new_right);
                    // さらに左に回転
                    (rotate_left(root), is_found)
                }
                Ordering::Greater => {
                    // 孫をsplay
                    let right_right = replace(&mut right.as_deref_mut().unwrap().right, None);
                    let (mut new_right_right, is_found) = splay_inner(right_right, key);
                    swap(
                        &mut right.as_deref_mut().unwrap().right,
                        &mut new_right_right,
                    );
                    // 親を左に回転
                    let tmp_child = rotate_left(root);
                    // さらに左に回転
                    (rotate_left(tmp_child), is_found)
                }
            }
        }
    }
}

/// ### 右回転
/// ```not-rust
///        Y                      X    
///       / \       right        / \   
///      X   C  === rotate ==>  A   Y  
///     / \                        / \
///    A   B                      B   C
/// ```
fn rotate_right<T: Ord, U>(root: Option<Box<Node<T, U>>>) -> Option<Box<Node<T, U>>> {
    if let Some(mut root) = root {
        if let Some(mut new_root) = root.left {
            root.left = new_root.right;
            new_root.right = Some(root);
            Some(new_root)
        } else {
            Some(root)
        }
    } else {
        None
    }
}

/// ### 左回転
/// ```not-rust
///      X                          Y  
///     / \         left           / \
///    A   Y    === rotate ==>    X   C
///       / \                    / \   
///      B   C                  A   B  
/// ```
fn rotate_left<T: Ord, U>(root: Option<Box<Node<T, U>>>) -> Option<Box<Node<T, U>>> {
    if let Some(mut root) = root {
        if let Some(mut new_root) = root.right {
            root.right = new_root.left;
            new_root.left = Some(root);
            Some(new_root)
        } else {
            Some(root)
        }
    } else {
        None
    }
}

// ----- Debug -----
impl<T, U> Debug for SplayTree<T, U>
where
    T: Ord + Debug,
    U: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_inner(f, &self.root, 0);
        Ok(())
    }
}

/// 再帰的に表示
fn fmt_inner<T, U>(f: &mut std::fmt::Formatter<'_>, node: &Option<Box<Node<T, U>>>, depth: usize)
where
    T: Ord + Debug,
    U: Debug,
{
    match node {
        Some(ref node) => {
            fmt_inner(f, &node.left, depth + 1);
            writeln!(
                f,
                "{}(key: {:?}, value: {:?})",
                " ".repeat(depth * 2),
                node.key,
                node.value
            );
            fmt_inner(f, &node.right, depth + 1);
        }
        None => {}
    }
}

// ----- test -----
#[cfg(test)]
mod test_splay_tree_util {
    use super::*;
    use crate::tree;

    #[test]
    fn test_rotate() {
        let mut tree = SplayTree::new();

        tree.root = tree! {
            key: 4,
            value: "1st",
            left: tree! {
                key: 2,
                value: "2nd",
                left: tree! {
                    key: 1,
                    value: "3rd",
                },
                right: tree! {
                    key: 3,
                    value: "4th",
                }
            },
            right: tree! {
                key: 5,
                value: "5th"
            }
        };

        println!("----- 回転前 -----");
        println!("{:?}", &tree);

        // ## 右回転のテスト
        // 右回転
        tree.root = rotate_left(tree.root);

        println!("----- 右回転 -----");
        println!("{:?}", &tree);

        // さらに右回転
        tree.root = rotate_left(tree.root);

        println!("----- 右回転 -----");
        println!("{:?}", &tree);

        // さらに右回転
        tree.root = rotate_left(tree.root);

        println!("----- 右回転 -----");
        println!("{:?}", &tree);

        // ## 左回転のテスト
        // 左回転
        tree.root = rotate_right(tree.root);

        println!("----- 左回転 -----");
        println!("{:?}", &tree);

        // さらに左回転
        tree.root = rotate_right(tree.root);

        println!("----- 左回転 -----");
        println!("{:?}", &tree);

        // さらに左回転
        tree.root = rotate_right(tree.root);

        println!("----- 左回転 -----");
        println!("{:?}", &tree);
    }
}
