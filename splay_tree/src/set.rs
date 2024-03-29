#![allow(unused_must_use)]

use std::iter::FromIterator;
use std::mem::{replace, swap};
use std::{cmp::Ordering, fmt::Debug};

/// # Node
#[derive(Debug)]
pub struct Node<T: Ord> {
    pub key: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn new(key: T) -> Self {
        Self {
            key,
            left: None,
            right: None,
        }
    }
}

/// # SplayTreeSet
/// スプレー木のクラス
pub struct SplayTreeSet<T: Ord> {
    size: usize,
    pub root: Option<Box<Node<T>>>,
}

impl<T> SplayTreeSet<T>
where
    T: Ord + Clone,
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

    /// ## get
    /// 値の検索を行う
    /// ### 戻り値
    /// - `Option<&T>`: キーに紐づいた値
    pub fn get(&mut self, key: &T) -> Option<&T> {
        if self.splay(key) {
            Some(&self.root.as_deref().unwrap().key)
        } else {
            None
        }
    }

    /// ## insert
    /// 値の挿入を行う。
    /// すでに同じキーが存在した場合は値を置き換えて前の値を返す。
    /// ### 戻り値
    /// - `Option<T>`: 以前の値
    pub fn insert(&mut self, key: T) -> Option<T> {
        // rootの取り出し
        let root = self.root.take();
        // splay操作
        let (mut tmp_root, is_found) = splay_inner(root, &key);
        if is_found {
            self.root = tmp_root;
            let res = replace(&mut self.root.as_deref_mut().unwrap().key, key);
            return Some(res);
        }
        // 挿入
        self.root = Some(Box::new(Node::new(key.clone())));
        if tmp_root.is_some() {
            match key.cmp(&tmp_root.as_deref().unwrap().key) {
                Ordering::Equal => unreachable!(),
                Ordering::Less => {
                    let mut new_left = tmp_root.as_deref_mut().unwrap().left.take();
                    swap(&mut self.root.as_deref_mut().unwrap().left, &mut new_left);
                    swap(&mut self.root.as_deref_mut().unwrap().right, &mut tmp_root);
                }
                Ordering::Greater => {
                    let mut new_right = tmp_root.as_deref_mut().unwrap().right.take();
                    swap(&mut self.root.as_deref_mut().unwrap().right, &mut new_right);
                    swap(&mut self.root.as_deref_mut().unwrap().left, &mut tmp_root);
                }
            }
        }
        // 要素数の更新
        self.size += 1;
        None
    }

    /// ## delete
    /// 値の削除
    /// ### 戻り値
    /// - `Option<T>`: 削除された値
    pub fn delete(&mut self, key: &T) -> Option<T> {
        // rootの取り出し
        let root = self.root.take();
        // splay操作
        let (mut tmp_root, is_found) = splay_inner(root, key);
        if !is_found {
            self.root = tmp_root;
            return None;
        }
        // 削除
        if tmp_root.as_deref().unwrap().left.is_none() {
            swap(&mut self.root, &mut tmp_root.as_deref_mut().unwrap().right);
        } else {
            let root_left = tmp_root.as_deref_mut().unwrap().left.take();
            swap(&mut self.root, &mut splay_inner(root_left, key).0);
            swap(
                &mut self.root.as_deref_mut().unwrap().right,
                &mut tmp_root.as_deref_mut().unwrap().right,
            );
        }
        let deleted = tmp_root.take();
        // 要素数の更新
        self.size -= 1;
        Some(deleted.unwrap().key)
    }

    /// ## splay
    /// スプレー操作をおこなう
    /// ### 戻り値
    /// - `bool`：要素が存在したかどうか
    pub fn splay(&mut self, key: &T) -> bool {
        // 根の取り出し
        let root = self.root.take();
        // スプレー操作
        let (new_root, is_found) = splay_inner(root, key);
        self.root = new_root;
        is_found
    }

    /// ## to_vec
    /// 要素を順にVecとして取り出す
    pub fn to_vec(&self) -> Vec<&T> {
        let mut res = vec![];
        traverse(&self.root, &mut res);
        res
    }
}

/// ## traverse
/// 順に取り出す
fn traverse<'a, T: Ord>(root: &'a Option<Box<Node<T>>>, res: &mut Vec<&'a T>) {
    if root.is_none() {
        return;
    }
    // 左の子を探索
    traverse(&root.as_ref().unwrap().left, res);
    // 値を追加
    res.push(&root.as_ref().unwrap().key);
    // 右の子を探索
    traverse(&root.as_ref().unwrap().right, res);
}

/// ## splay_inner
/// splay操作を行う
/// ### 戻り値
/// - `Option<Box<Node<T>>>`：新しく根となるノード
/// - `bool`：目的の値が存在したかどうか
fn splay_inner<T: Ord>(mut root: Option<Box<Node<T>>>, key: &T) -> (Option<Box<Node<T>>>, bool) {
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
                    let left_left = left.as_deref_mut().unwrap().left.take();
                    let (mut new_left_left, is_found) = splay_inner(left_left, key);
                    swap(&mut left.as_deref_mut().unwrap().left, &mut new_left_left);
                    // 親を右に回転
                    let tmp_child = rotate_right(root);
                    // さらに右に回転
                    (rotate_right(tmp_child), is_found)
                }
                Ordering::Greater => {
                    // 孫をsplay
                    let left_right = left.as_deref_mut().unwrap().right.take();
                    let (mut new_left_right, is_found) = splay_inner(left_right, key);
                    swap(&mut left.as_deref_mut().unwrap().right, &mut new_left_right);
                    // 左の子を左に回転
                    let left = root.as_deref_mut().unwrap().left.take();
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
                    let right_left = right.as_deref_mut().unwrap().left.take();
                    let (mut new_right_left, is_found) = splay_inner(right_left, key);
                    swap(&mut right.as_deref_mut().unwrap().left, &mut new_right_left);
                    // 右の子を右に回転
                    let right = root.as_deref_mut().unwrap().right.take();
                    let mut new_right = rotate_right(right);
                    swap(&mut root.as_deref_mut().unwrap().right, &mut new_right);
                    // さらに左に回転
                    (rotate_left(root), is_found)
                }
                Ordering::Greater => {
                    // 孫をsplay
                    let right_right = right.as_deref_mut().unwrap().right.take();
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

/// ## 右回転
/// ```text
///        Y                      X
///       / \       right        / \
///      X   C  === rotate ==>  A   Y
///     / \                        / \
///    A   B                      B   C
/// ```
fn rotate_right<T: Ord>(root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
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

/// ## 左回転
/// ```text
///      X                          Y
///     / \         left           / \
///    A   Y    === rotate ==>    X   C
///       / \                    / \
///      B   C                  A   B
/// ```
fn rotate_left<T: Ord>(root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
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

// ----- FromIterator -----
impl<T: Ord + Clone> FromIterator<T> for SplayTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res = SplayTreeSet::new();
        for item in iter {
            res.insert(item);
        }
        res
    }
}

// ----- Debug -----
impl<T: Ord + Debug> Debug for SplayTreeSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_inner(f, &self.root, 0);
        Ok(())
    }
}

/// 再帰的に表示
fn fmt_inner<T>(f: &mut std::fmt::Formatter<'_>, node: &Option<Box<Node<T>>>, depth: usize)
where
    T: Ord + Debug,
{
    match node {
        Some(ref node) => {
            fmt_inner(f, &node.left, depth + 1);
            writeln!(f, "{}{:?}", " ".repeat(depth * 2), node.key);
            fmt_inner(f, &node.right, depth + 1);
        }
        None => {}
    }
}
