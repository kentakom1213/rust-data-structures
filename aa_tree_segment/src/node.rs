//! セグ木のノード

#![allow(non_snake_case)]

use crate::alg::Monoid;
use std::{cmp::Ordering, fmt::Debug};

/// AA木のノード
pub type Node<K, M> = Option<Box<NodeInner<K, M>>>;

pub struct NodeInner<K: Ord, M: Monoid> {
    pub key: K,
    pub value: M::Val,
    /// 部分木の大きさ
    pub size: usize,
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
            value,
            size: 1,
            level: 1,
            left: None,
            right: None,
        }))
    }

    /// ノードの値を再計算する
    fn eval(&mut self) {
        // // ノードの値を再計算
        // self.value = match (&self.left, &self.right) {
        //     (Some(l), Some(r)) => M::op(&l.value, &r.value),
        //     (Some(l), _) => l.value.clone(),
        //     (_, Some(r)) => r.value.clone(),
        //     _ => M::E,
        // };
        // 部分木のサイズを再計算
        self.size = match (&self.left, &self.right) {
            (Some(l), Some(r)) => l.size + r.size,
            (Some(l), _) => l.size,
            (_, Some(r)) => r.size,
            _ => 0,
        } + 1;
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

/// 区間 `[l,r]` 中のノードの値を集約する
pub fn get_range<K: Ord, M: Monoid>(root: Node<K, M>, l: &K, r: &K) -> M::Val {
    todo!()
}

/// 値 `key` に `value` を挿入する
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
        let mut seg = None;

        // 0 <- 5
        seg = insert::<_, Add>(seg, 0, 5);
        print_as_binary_tree(&seg);

        // 5 <- 8
        seg = insert(seg, 5, 8);
        print_as_binary_tree(&seg);

        // 3 <- 3
        seg = insert(seg, 3, 3);
        print_as_binary_tree(&seg);
    }
}
