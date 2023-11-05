#![allow(unused_must_use)]

use std::iter::FromIterator;
use std::mem::swap;
use std::{cmp::Ordering, fmt::Debug};

/// # Node
#[derive(Debug, Clone)]
pub struct Node<T: Ord + Debug> {
    pub key: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T: Ord + Debug> Node<T> {
    pub fn new(key: T) -> Self {
        Self {
            key,
            left: None,
            right: None,
        }
    }
}

/// # SplayTreeMultiSet
/// スプレー木のクラス
pub struct SplayTreeMultiSet<T: Ord + Debug> {
    size: usize,
    pub root: Option<Box<Node<T>>>,
}

impl<T> SplayTreeMultiSet<T>
where
    T: Ord + Clone + Debug,
{
    /// `a <= b`の値を返す
    #[inline]
    fn le(a: &T, b: &T) -> bool {
        matches!(a.cmp(b), Ordering::Less | Ordering::Equal)
    }

    /// `a < b`の値を返す
    #[inline]
    fn lt(a: &T, b: &T) -> bool {
        matches!(a.cmp(b), Ordering::Less)
    }

    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// ## get
    /// 値の検索を行う
    /// ### 戻り値
    /// - `Option<&T>`: キーに紐づいた値
    pub fn get(&mut self, key: &T) -> Option<&T> {
        if self.lower_bound_splay(key) && &self.root.as_ref().unwrap().key == key {
            Some(&self.root.as_ref().unwrap().key)
        } else {
            None
        }
    }

    /// ## insert
    /// 値の挿入を行う。
    pub fn insert(&mut self, key: T) {
        // rootの取り出し
        let root = self.root.take();
        // splay操作（一番右の要素）
        let (mut tmp_root, _) = binary_search_mut(root, &key, Self::lt);
        // 挿入
        self.root = Some(Box::new(Node::new(key.clone())));
        if tmp_root.is_some() {
            match key.cmp(&tmp_root.as_ref().unwrap().key) {
                Ordering::Less => {
                    let mut new_left = tmp_root.as_mut().unwrap().left.take();
                    swap(&mut self.root.as_mut().unwrap().left, &mut new_left);
                    swap(&mut self.root.as_mut().unwrap().right, &mut tmp_root);
                }
                Ordering::Equal | Ordering::Greater => {
                    let mut new_right = tmp_root.as_mut().unwrap().right.take();
                    swap(&mut self.root.as_mut().unwrap().right, &mut new_right);
                    swap(&mut self.root.as_mut().unwrap().left, &mut tmp_root);
                }
            }
        }
        // 要素数の更新
        self.size += 1;
    }

    /// ## delete
    /// 値の削除
    /// ### 戻り値
    /// - `Option<T>`: 削除された値
    pub fn delete(&mut self, key: &T) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        // rootの取り出し
        let root = self.root.take();
        // splay操作
        // tmp_root := keyより真に大きいノードのうち最小のもの
        let (mut tmp_root, _) = binary_search_mut(root, key, Self::lt);
        // 値の存在判定
        if &tmp_root.as_ref().unwrap().key == key {
            // 値が根にあるとき（何もしない）
        } else if tmp_root
            .as_ref()
            .unwrap()
            .left
            .as_deref()
            .map(|k| &k.key == key)
            == Some(true)
        {
            // 値が左の子にあるとき（右回転）
            tmp_root = rotate_right(tmp_root);
        } else {
            // 値がないとき（Noneを返す）
            self.root = tmp_root;
            return None;
        }
        // 削除
        if tmp_root.as_ref().unwrap().left.is_none() {
            swap(&mut self.root, &mut tmp_root.as_mut().unwrap().right);
        } else {
            let root_left = tmp_root.as_mut().unwrap().left.take();
            // 左の子のうち最大の要素を新しい根に
            swap(
                &mut self.root,
                &mut binary_search_mut(root_left, key, Self::lt).0,
            );
            // 根の右側に子を付け替える
            swap(
                &mut self.root.as_mut().unwrap().right,
                &mut tmp_root.as_mut().unwrap().right,
            );
        }
        let deleted = tmp_root.take();
        // 要素数の更新
        self.size -= 1;
        Some(deleted.unwrap().key)
    }

    /// ## lower_bound_splay
    /// - lower_boundを求める
    pub fn lower_bound_splay(&mut self, key: &T) -> bool {
        // 根の取り出し
        let root = self.root.take();
        // スプレー操作
        let (new_root, is_found) = binary_search_mut(root, key, Self::le);
        self.root = new_root;
        is_found
    }

    /// ## upper_bound_splay
    /// - upper_boundを求める
    pub fn upper_bound_splay(&mut self, key: &T) -> bool {
        // 根の取り出し
        let root = self.root.take();
        // スプレー操作
        let (new_root, is_found) = binary_search_mut(root, key, Self::lt);
        self.root = new_root;
        is_found
    }

    /// ## lower_bound
    /// - lower_boundを求める
    pub fn lower_bound(&mut self, key: &T) -> &Option<Box<Node<T>>> {
        binary_search(&self.root, key, Self::le)
    }

    /// ## upper_bound
    /// - upper_boundを求める
    pub fn upper_bound(&mut self, key: &T) -> &Option<Box<Node<T>>> {
        binary_search(&self.root, key, Self::lt)
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
fn traverse<'a, T: Ord + Debug>(root: &'a Option<Box<Node<T>>>, res: &mut Vec<&'a T>) {
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

/// ## binary_search
/// 比較関数`compare`を引数にとり、条件を満たす最小のノードを返す
fn binary_search<'a, T, C>(
    root: &'a Option<Box<Node<T>>>,
    key: &T,
    compare: C,
) -> &'a Option<Box<Node<T>>>
where
    T: Ord + Debug,
    C: Fn(&T, &T) -> bool,
{
    if root.is_none() {
        return root;
    }
    if compare(key, &root.as_ref().unwrap().key) {
        let left = &root.as_ref().unwrap().left;
        if left.is_none() {
            return root;
        }
        if compare(key, &left.as_ref().unwrap().key) {
            let leftleft = &left.as_ref().unwrap().left;
            let tmp = binary_search(leftleft, key, compare);
            if tmp.is_none() {
                left
            } else {
                tmp
            }
        } else {
            let leftright = &left.as_ref().unwrap().right;
            let tmp = binary_search(leftright, key, compare);
            if tmp.is_none() {
                root
            } else {
                tmp
            }
        }
    } else {
        let right = &root.as_ref().unwrap().right;
        if right.is_none() {
            return right;
        }
        if compare(key, &right.as_ref().unwrap().key) {
            let rightleft = &right.as_ref().unwrap().left;
            let tmp = binary_search(rightleft, key, compare);
            if tmp.is_none() {
                right
            } else {
                tmp
            }
        } else {
            let rightright = &right.as_ref().unwrap().right;
            binary_search(rightright, key, compare)
        }
    }
}

/// ## binary_search_mut
/// 比較関数`compare`を引数にとり、条件を満たす最小のノードを返す
fn binary_search_mut<T, C>(
    mut root: Option<Box<Node<T>>>,
    key: &T,
    compare: C,
) -> (Option<Box<Node<T>>>, bool)
where
    T: Ord + Debug,
    C: Fn(&T, &T) -> bool,
{
    if root.is_none() {
        return (root, false);
    }
    if compare(key, &root.as_ref().unwrap().key) {
        let left = &mut root.as_mut().unwrap().left;
        if left.is_none() {
            return (root, true);
        }
        if compare(key, &left.as_ref().unwrap().key) {
            let leftleft = left.as_mut().unwrap().left.take();
            let (mut tmp, is_found) = binary_search_mut(leftleft, key, compare);
            // 戻す
            swap(&mut left.as_mut().unwrap().left, &mut tmp);
            // 親を右に回転
            let tmp_left = rotate_right(root);
            if !is_found {
                return (tmp_left, true);
            }
            // さらに右回転
            (rotate_right(tmp_left), true)
        } else {
            let leftright = left.as_mut().unwrap().right.take();
            let (mut new_leftright, is_found) = binary_search_mut(leftright, key, compare);
            // 戻す
            swap(&mut left.as_mut().unwrap().right, &mut new_leftright);
            // root->left->rightがNoneでないとき
            if !is_found {
                return (root, true);
            }
            // 左の子を左回転
            let left = root.as_mut().unwrap().left.take();
            let mut tmp_child = rotate_left(left);
            swap(&mut root.as_mut().unwrap().left, &mut tmp_child);
            // 親を右回転
            (rotate_right(root), true)
        }
    } else {
        let right = &mut root.as_mut().unwrap().right;
        if right.is_none() {
            return (root, false);
        }
        if compare(key, &right.as_ref().unwrap().key) {
            let rightleft = right.as_mut().unwrap().left.take();
            let (mut tmp, is_found) = binary_search_mut(rightleft, key, compare);
            // 戻す
            swap(&mut right.as_mut().unwrap().left, &mut tmp);
            if is_found {
                // 右の子を右回転
                let right = root.as_mut().unwrap().right.take();
                let mut tmp_child = rotate_right(right);
                swap(&mut root.as_mut().unwrap().right, &mut tmp_child);
            }
            // 親を左回転
            (rotate_left(root), true)
        } else {
            let rightright = right.as_mut().unwrap().right.take();
            let (mut tmp, is_found) = binary_search_mut(rightright, key, compare);
            // 戻す
            swap(&mut right.as_mut().unwrap().right, &mut tmp);
            // 親を左回転
            let tmp_child = rotate_left(root);
            // さらに左回転
            (rotate_left(tmp_child), is_found)
        }
    }
}

/// ## 右回転
/// ```not-rust
///        Y                      X
///       / \       right        / \
///      X   C  === rotate ==>  A   Y
///     / \                        / \
///    A   B                      B   C
/// ```
fn rotate_right<T: Ord + Debug>(root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
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
/// ```not-rust
///      X                          Y
///     / \         left           / \
///    A   Y    === rotate ==>    X   C
///       / \                    / \
///      B   C                  A   B
/// ```
fn rotate_left<T: Ord + Debug>(root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
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
impl<T: Ord + Clone + Debug> FromIterator<T> for SplayTreeMultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res = SplayTreeMultiSet::new();
        for item in iter {
            res.insert(item);
        }
        res
    }
}

// ----- Debug -----
impl<T: Ord + Debug> Debug for SplayTreeMultiSet<T> {
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