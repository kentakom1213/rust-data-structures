//! AA木による動的セグ木
//! - 遅延評価なし

use crate::{alg::Monoid, node::Node};

/// 動的セグメント木
/// - 平行2分木（AA木）
/// - 遅延評価なし
pub struct DynamicSegmentTree<K: Ord, M: Monoid> {
    pub root: Node<K, M>,
}
