//! セグ木上の二分探索

use crate::{alg::Monoid, node::Node};

/// `f(l,r)=true`を満たす最大の`r`を求める
/// - `f(l,r) -> bool`：区間 `l`,`r` が条件を満たすかを判定する**単調な**関数
fn max_right<K, M, F>(root: Node<K, M>, l: &M::Val, f: F)
where
    K: Ord,
    M: Monoid,
    F: Fn(&K, &K) -> bool,
{
    todo!()
}

/// `f(l,r)=true`を満たす最小の`l`を求める
/// - `f(l,r) -> bool`：区間 `l`,`r` が条件を満たすかを判定する**単調な**関数
fn min_left<K, M, F>(root: Node<K, M>, r: &M::Val, f: F)
where
    K: Ord,
    M: Monoid,
    F: Fn(&K, &K) -> bool,
{
    todo!()
}
