//! セグ木のノード

#![allow(non_snake_case)]

use crate::alg::Monoid;
use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::Bound::{self, *},
};

/// AA木のノード
pub type Node<K, M> = Option<Box<NodeInner<K, M>>>;

pub struct NodeInner<K: Ord, M: Monoid> {
    /// キー
    pub key: K,
    /// ノードが持つ値
    pub value: M::Val,
    /// 部分木を集約した値
    pub sum: M::Val,
    // /// 部分木の大きさ
    // pub size: usize,
    /// ノードの高さ
    pub level: usize,
    pub left: Node<K, M>,
    pub right: Node<K, M>,
}

impl<K: Ord, M: Monoid> NodeInner<K, M> {
    /// ノードの作成
    pub fn new(key: K, value: M::Val) -> Node<K, M> {
        Some(Box::new(NodeInner {
            key,
            value: value.clone(),
            sum: value,
            // size: 1,
            level: 1,
            left: None,
            right: None,
        }))
    }

    /// ノードの値を再計算する
    fn eval(&mut self) {
        // ノードの値を再計算
        self.sum = match (&self.left, &self.right) {
            (Some(l), Some(r)) => M::op(&M::op(&l.sum, &self.value), &r.sum),
            (Some(l), _) => M::op(&l.sum, &self.value),
            (_, Some(r)) => M::op(&self.value, &r.sum),
            _ => self.value.clone(),
        };
        // // 部分木のサイズを再計算
        // self.size = match (&self.left, &self.right) {
        //     (Some(l), Some(r)) => l.size + r.size,
        //     (Some(l), _) => l.size,
        //     (_, Some(r)) => r.size,
        //     _ => 0,
        // } + 1;
    }
}

impl<K, M> Debug for NodeInner<K, M>
where
    K: Ord + Debug,
    M: Monoid,
    M::Val: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ key: {:?}, value: {:?}, sum: {:?} }}",
            self.key, self.value, self.sum
        )
    }
}

/// skew操作
/// ```text
///   |        ⇓           ⇓        
/// 2 |    L ← T           L → T    
///   |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
/// 1 |  A   B   R       A   B   R  
/// ```
fn skew<K: Ord, M: Monoid>(node: Node<K, M>) -> Node<K, M> {
    let Some(mut T) = node else {
        return None;
    };
    if T.left.is_none() {
        Some(T)
    } else if T.level == T.left.as_ref().unwrap().level {
        let mut L = T.left.unwrap();
        // Tを更新
        T.left = L.right;
        T.eval();
        // Lを更新
        L.right = Some(T);
        L.eval();
        Some(L)
    } else {
        Some(T)
    }
}

/// split操作
/// ```text
///   |                         ⇓    
/// 3 |                         R    
///   |    ⇓                   ↙ ↘   
/// 2 |    T → R → X   ==>    T   X  
///   |   ↙   ↙              ↙ ↘     
/// 1 |  A   B              A   B    
/// ```
fn split<K: Ord, M: Monoid>(node: Node<K, M>) -> Node<K, M> {
    let Some(mut T) = node else {
        return None;
    };
    if T.right.is_none() || T.right.as_ref().unwrap().right.is_none() {
        Some(T)
    } else if T.level == T.right.as_ref().unwrap().right.as_ref().unwrap().level {
        let mut R = T.right.unwrap();
        // Tを更新
        T.right = R.left;
        T.eval();
        // Rを更新
        R.left = Some(T);
        R.eval();
        R.level += 1; // Rのレベルを1上げる
        Some(R)
    } else {
        Some(T)
    }
}

/// 値 `key` を持つノードの不変参照を取得する
pub fn get<'a, K: Ord, M: Monoid>(root: &'a Node<K, M>, key: &K) -> Option<&'a NodeInner<K, M>> {
    let Some(T) = root else {
        return None;
    };
    match key.cmp(&T.key) {
        Ordering::Less => get(&T.left, key),
        Ordering::Greater => get(&T.right, key),
        Ordering::Equal => Some(T),
    }
}

/// 値 `key` を持つノードの可変参照を取得する
pub fn get_mut<K: Ord, M: Monoid>(root: Node<K, M>, key: &K) {
    todo!()
}

/// 区間 `[l,r)` 中のノードの値を集約する
pub fn get_range<K: Ord, M: Monoid>(root: &Node<K, M>, l: &K, r: &K, begin: &K, end: &K) -> M::Val {
    let Some(T) = root else {
        return M::E;
    };
    // 区間を含まない
    if end <= l || r <= begin {
        M::E
    }
    // 区間を包含する
    else if l <= begin && end <= r {
        T.sum.clone()
    }
    // 区間が一部重なる
    else {
        let mid = &T.key;

        // 右の子だけ範囲内
        if mid < l {
            get_range(&T.right, l, r, mid, end)
        }
        // 自分も範囲内
        else if mid < r {
            let l_val = &get_range(&T.left, l, r, begin, mid);
            let m_val = &T.value;
            let r_val = &get_range(&T.right, l, r, mid, end);
            M::op(&M::op(l_val, m_val), r_val)
        }
        // 左の子だけ範囲内
        else {
            get_range(&T.left, l, r, begin, mid)
        }
    }
}

/// 値 `key` に `value` を挿入する
/// - 値がすでに存在する場合には更新する
pub fn insert<K: Ord, M: Monoid>(root: Node<K, M>, key: K, value: M::Val) -> Node<K, M> {
    let Some(mut T) = root else {
        return NodeInner::new(key, value);
    };
    // 挿入
    match key.cmp(&T.key) {
        Ordering::Less => {
            T.left = insert(T.left, key, value);
        }
        Ordering::Greater => {
            T.right = insert(T.right, key, value);
        }
        Ordering::Equal => {
            T.value = value;
        }
    }
    // ノードの評価
    T.eval();
    // 再平衡化
    let mut root = Some(T);
    root = skew(root);
    root = split(root);
    root
}

// ========== TEST ===========
#[cfg(test)]
mod test_segment_tree {
    use crate::{alg::monoids::Add, print_util::print_as_binary_tree};

    use super::*;

    #[test]
    fn test_insert() {
        let mut seg: Option<Box<NodeInner<i32, Add>>> = None;

        assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
        assert_eq!(get_range(&seg, &0, &10, &0, &10), 0);
        assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
        assert_eq!(get_range(&seg, &2, &8, &0, &10), 0);
        assert_eq!(get_range(&seg, &3, &6, &0, &10), 0);
        assert_eq!(get_range(&seg, &4, &9, &0, &10), 0);

        // [(2: 5)]
        seg = insert(seg, 2, 5);
        print_as_binary_tree(&seg);

        assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
        assert_eq!(get_range(&seg, &0, &10, &0, &10), 5);
        assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
        assert_eq!(get_range(&seg, &2, &8, &0, &10), 5);
        assert_eq!(get_range(&seg, &3, &6, &0, &10), 0);
        assert_eq!(get_range(&seg, &4, &9, &0, &10), 0);

        // [(2: 5), (5: 8)]
        seg = insert(seg, 5, 8);
        print_as_binary_tree(&seg);

        assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
        assert_eq!(get_range(&seg, &0, &10, &0, &10), 13);
        assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
        assert_eq!(get_range(&seg, &2, &8, &0, &10), 13);
        assert_eq!(get_range(&seg, &3, &6, &0, &10), 8);
        assert_eq!(get_range(&seg, &4, &9, &0, &10), 8);

        // [(2: 5), (3: 3), (5: 8)]
        seg = insert(seg, 3, 3);
        print_as_binary_tree(&seg);

        assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
        assert_eq!(get_range(&seg, &0, &10, &0, &10), 16);
        assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
        assert_eq!(get_range(&seg, &2, &8, &0, &10), 16);
        assert_eq!(get_range(&seg, &3, &6, &0, &10), 11);
        assert_eq!(get_range(&seg, &4, &9, &0, &10), 8);

        // [(2: 5), (3: 3), (5: 8), (8: 1)]
        seg = insert(seg, 8, 1);
        print_as_binary_tree(&seg);

        assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
        assert_eq!(get_range(&seg, &0, &10, &0, &10), 17);
        assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
        assert_eq!(get_range(&seg, &2, &8, &0, &10), 16);
        assert_eq!(get_range(&seg, &3, &6, &0, &10), 11);
        assert_eq!(get_range(&seg, &4, &9, &0, &10), 9);

        // [(2: 5), (3: 3), (4: 6), (5: 8), (8: 1)]
        seg = insert(seg, 4, 6);
        print_as_binary_tree(&seg);

        assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
        assert_eq!(get_range(&seg, &0, &10, &0, &10), 23);
        assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
        assert_eq!(get_range(&seg, &2, &8, &0, &10), 22);
        assert_eq!(get_range(&seg, &3, &6, &0, &10), 17);
        assert_eq!(get_range(&seg, &4, &9, &0, &10), 15);
    }

    /// 文字列
    struct Str;
    impl Monoid for Str {
        type Val = String;
        const E: Self::Val = String::new();
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left.to_string() + right
        }
    }

    #[test]
    fn test_noncommutative() {
        let mut seg: Node<usize, Str> = None;

        for (i, c) in ('A'..='G').enumerate() {
            seg = insert(seg, i, c.to_string());
            print_as_binary_tree(&seg);
        }

        assert_eq!(&get_range(&seg, &5, &6, &0, &7), "F");
        assert_eq!(&get_range(&seg, &4, &20, &0, &100), "EFG");
        assert_eq!(&get_range(&seg, &0, &7, &0, &9), "ABCDEFG");
        assert_eq!(&get_range(&seg, &1, &5, &0, &9), "BCDE");
        assert_eq!(&get_range(&seg, &0, &1, &0, &9), "A");
        assert_eq!(&get_range(&seg, &6, &7, &0, &9), "G");
    }
}
