//! セグ木のノード

#![allow(non_snake_case)]

use crate::lazy_alg::ExtMonoid;
use std::{
    cmp::Ordering,
    fmt::Debug,
    mem,
    ops::Bound::{self, *},
};

/// AA木のノード
pub type LazyNode<K, E> = Option<Box<LazyNodeInner<K, E>>>;

pub struct LazyNodeInner<K: Ord, E: ExtMonoid> {
    /// キー
    pub key: K,
    /// ノードが持つ値
    pub value: E::X,
    /// 部分木を集約した値
    pub sum: E::X,
    /// 遅延値
    pub lazy: E::M,
    /// ノードの高さ
    pub level: usize,
    pub left: LazyNode<K, E>,
    pub right: LazyNode<K, E>,
}

impl<K: Ord, E: ExtMonoid> LazyNodeInner<K, E> {
    /// ノードの作成
    pub fn new(key: K, value: E::X) -> LazyNode<K, E> {
        Some(Box::new(LazyNodeInner {
            key,
            value: value.clone(),
            sum: value,
            lazy: E::IM,
            level: 1,
            left: None,
            right: None,
        }))
    }

    /// ノードの値を再計算する
    fn eval(&mut self) {
        // ノードの値を再計算
        self.sum = match (&self.left, &self.right) {
            (Some(l), Some(r)) => E::operate_x(&E::operate_x(&l.sum, &self.value), &r.sum),
            (Some(l), _) => E::operate_x(&l.sum, &self.value),
            (_, Some(r)) => E::operate_x(&self.value, &r.sum),
            _ => self.value.clone(),
        };
    }
}

impl<K, E> Debug for LazyNodeInner<K, E>
where
    K: Ord + Debug,
    E: ExtMonoid,
    E::X: Debug,
    E::M: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazyNode")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("sum", &self.sum)
            .field("lazy", &self.lazy)
            .finish()
    }
}

/// skew操作
/// ```text
///   |        ⇓           ⇓        
/// 2 |    L ← T           L → T    
///   |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
/// 1 |  A   B   R       A   B   R  
/// ```
fn skew<K: Ord, E: ExtMonoid>(node: LazyNode<K, E>) -> LazyNode<K, E> {
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
fn split<K: Ord, E: ExtMonoid>(node: LazyNode<K, E>) -> LazyNode<K, E> {
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
pub fn get<'a, K: Ord, E: ExtMonoid>(
    root: &'a LazyNode<K, E>,
    key: &K,
) -> Option<&'a LazyNodeInner<K, E>> {
    let Some(T) = root else {
        return None;
    };
    match key.cmp(&T.key) {
        Ordering::Less => get(&T.left, key),
        Ordering::Greater => get(&T.right, key),
        Ordering::Equal => Some(T),
    }
}

/// 区間
type Segment<K> = (Bound<K>, Bound<K>);

/// 区間 `x` と `y` が共通部分を持たないか判定
fn has_no_intersection<K: Ord>((l, r): Segment<&K>, (begin, end): Segment<&K>) -> bool {
    (match (r, begin) {
        (Included(r), Included(b)) => r < b,
        (Included(r), Excluded(b)) => r <= b,
        (Excluded(r), Included(b)) => r <= b,
        (Excluded(r), Excluded(b)) => r <= b,
        _ => false,
    } || match (end, l) {
        (Included(e), Included(l)) => e < l,
        (Included(e), Excluded(l)) => e <= l,
        (Excluded(e), Included(l)) => e <= l,
        (Excluded(e), Excluded(l)) => e <= l,
        _ => false,
    })
}

/// 区間 `x`（引数1） が区間 `y`（引数2） を包含するか
fn includes<K: Ord>((l, r): Segment<&K>, (begin, end): Segment<&K>) -> bool {
    (match (l, begin) {
        (Unbounded, _) => true,
        (_, Unbounded) => false,
        (Included(l), Included(b)) => l <= b,
        (Included(l), Excluded(b)) => l <= b,
        (Excluded(l), Included(b)) => l < b,
        (Excluded(l), Excluded(b)) => l <= b,
    } && match (end, r) {
        (_, Unbounded) => true,
        (Unbounded, _) => false,
        (Included(e), Included(r)) => e <= r,
        (Included(e), Excluded(r)) => e < r,
        (Excluded(e), Included(r)) => e <= r,
        (Excluded(e), Excluded(r)) => e <= r,
    })
}

/// 区間 `[l,r)` 中のノードの値を集約する
pub fn get_range<K: Ord, E: ExtMonoid>(
    root: &LazyNode<K, E>,
    l: Bound<&K>,
    r: Bound<&K>,
    begin: Bound<&K>,
    end: Bound<&K>,
) -> E::X {
    let Some(T) = root else {
        return E::IX;
    };
    // 区間を含まない
    if has_no_intersection((l, r), (begin, end)) {
        E::IX
    }
    // 区間を包含する
    else if includes((l, r), (begin, end)) {
        T.sum.clone()
    }
    // 区間が一部重なる
    else {
        let mid = &T.key;
        let l_val = get_range(&T.left, l, r, begin, Excluded(mid));
        let m_val = if includes((l, r), (Included(mid), Included(mid))) {
            T.value.clone()
        } else {
            E::IX
        };
        let r_val = get_range(&T.right, l, r, Excluded(mid), end);
        E::operate_x(&E::operate_x(&l_val, &m_val), &r_val)
    }
}

/// 値 `key` に `value` を挿入する
/// - 値がすでに存在する場合には更新し，もとの値を返す
pub fn insert<K: Ord, E: ExtMonoid>(
    root: LazyNode<K, E>,
    key: K,
    value: E::X,
) -> (LazyNode<K, E>, Option<(K, E::X)>) {
    let Some(mut T) = root else {
        return (LazyNodeInner::new(key, value), None);
    };
    // 挿入
    let old_key_value = match key.cmp(&T.key) {
        Ordering::Less => {
            let (new_left, old_key_value) = insert(T.left, key, value);
            T.left = new_left;
            old_key_value
        }
        Ordering::Greater => {
            let (new_right, old_key_value) = insert(T.right, key, value);
            T.right = new_right;
            old_key_value
        }
        Ordering::Equal => Some((
            mem::replace(&mut T.key, key),
            mem::replace(&mut T.value, value),
        )),
    };
    // ノードの評価
    T.eval();
    // 再平衡化
    let mut root = Some(T);
    root = skew(root);
    root = split(root);
    (root, old_key_value)
}

/// 値 `key` をもつノードを削除し，削除されたノードを返す
/// - `root`：削除する木の根
pub fn delete<K: Ord, E: ExtMonoid>(
    root: LazyNode<K, E>,
    key: &K,
) -> (LazyNode<K, E>, Option<(K, E::X)>) {
    let Some(mut T) = root else {
        return (None, None);
    };
    let (mut new_root, old_key_value) = match key.cmp(&T.key) {
        Ordering::Less => {
            let (new_left, old_key_value) = delete(T.left, key);
            T.left = new_left;
            (Some(T), old_key_value)
        }
        Ordering::Greater => {
            let (new_right, old_key_value) = delete(T.right, key);
            T.right = new_right;
            (Some(T), old_key_value)
        }
        Ordering::Equal => {
            if T.left.is_none() {
                (T.right, Some((T.key, T.value)))
            } else if T.right.is_none() {
                (T.left, Some((T.key, T.value)))
            } else {
                // 左右の子を持つ場合，左の子の最大値を現在のノードに代入
                let (new_left, right_most) = delete_and_get_max(T.left.take());
                if let Some(L) = new_left {
                    T.left.replace(L);
                }
                let Some(right_most) = right_most else {
                    unreachable!("T.left is not None");
                };
                let old_key_value = (
                    mem::replace(&mut T.key, right_most.key),
                    mem::replace(&mut T.value, right_most.value),
                );
                (Some(T), Some(old_key_value))
            }
        }
    };
    // 評価
    if let Some(T) = &mut new_root {
        T.eval();
    }
    // バランスの修正
    let rebalanced = rebarance(new_root);
    (rebalanced, old_key_value)
}

/// 削除後の頂点を再平衡化
fn rebarance<K: Ord, E: ExtMonoid>(root: LazyNode<K, E>) -> LazyNode<K, E> {
    let Some(mut T) = root else {
        return None;
    };
    let left_level = T.left.as_ref().map_or(0, |node| node.level);
    let right_level = T.right.as_ref().map_or(0, |node| node.level);
    if left_level.min(right_level) < T.level - 1 {
        T.level -= 1;
        // 右が大きい場合，下げる
        if right_level > T.level {
            T.right.as_mut().unwrap().level = T.level;
        }
        // 同じレベルのノードをskew
        T = skew(Some(T)).unwrap();
        T.right = skew(T.right);
        if let Some(mut right) = T.right.take() {
            right.right = skew(right.right);
            T.right.replace(right);
        }
        // 同じレベルのノードをsplit
        T = split(Some(T)).unwrap();
        T.right = split(T.right);
        // ノードの再評価
        T.eval();
    }
    Some(T)
}

/// nodeを根とする木のうち，値が最大のものを削除する
/// - 戻り値：(新しい根, 削除されたノード)
fn delete_and_get_max<K: Ord, E: ExtMonoid>(
    root: LazyNode<K, E>,
) -> (LazyNode<K, E>, Option<LazyNodeInner<K, E>>) {
    let Some(mut T) = root else {
        return (None, None);
    };
    // 右の子の取り出し
    let (new_right, right_most) = delete_and_get_max(T.right.take());
    let Some(right_most) = right_most else {
        return (None, Some(*T));
    };
    if let Some(R) = new_right {
        T.right.replace(R);
    }
    // ノードを再評価
    T.eval();
    let mut new_root = Some(T);
    // 削除したので，再平衡化
    new_root = rebarance(new_root);
    (new_root, Some(right_most))
}
