//! セグ木上の二分探索

use crate::{alg::Monoid, node::Node};

/// `f(get_range(l,r))=true`を満たす最大の`r`を求める
/// - `f(x) -> bool`：値 `x` が条件を満たすかを判定する関数
/// ---
/// 関数 `f` が満たすべき条件
/// - `x = M::E` のとき `f(x) = true`
/// - `x < y` のとき，`f(x) >= f(y)`（単調性）
fn max_right<K, M, F>(root: &Node<K, M>, l: &K, f: F)
where
    K: Ord,
    M: Monoid,
    F: Fn(&M::Val) -> bool,
{
    todo!()
}

/// `f(get_range(l,r))=true`を満たす最小の`l`を求める
/// - `f(x) -> bool`：値 `x` が条件を満たすかを判定する関数
/// ---
/// 関数 `f` が満たすべき条件
/// - `x = M::E` のとき `f(x) = true`
/// - `x < y` のとき，`f(x) >= f(y)`（単調性）
fn min_left<K, M, F>(root: &Node<K, M>, r: &K, f: F)
where
    K: Ord,
    M: Monoid,
    F: Fn(&M::Val) -> bool,
{
    todo!()
}
