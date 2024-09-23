//! 多重集合

use crate::node::NodePtr;

/// Multiset
/// - 多重集合
pub struct Multiset<K: Ord> {
    root: NodePtr<K, usize>,
    size: usize,
}

impl<K: Ord> Multiset<K> {
    /// 新規作成
    fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    
}
