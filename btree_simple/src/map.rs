//! mapの構造体

use crate::NodePtr;

/// B木による連想配列
pub struct Map<const D: usize, K, V>
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    /// ルートノード
    pub root: Option<NodePtr<D, K, V>>,
}
