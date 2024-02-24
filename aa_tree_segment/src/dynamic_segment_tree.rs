//! AA木による動的セグ木
//! - 遅延評価なし

use crate::{
    alg::Monoid,
    node::{delete, get_range, insert, Node},
};

/// 動的セグメント木
/// - 平行2分木（AA木）
/// - 遅延評価なし
pub struct DynamicSegmentTree<K: Ord, M: Monoid> {
    pub root: Node<K, M>,
    size: usize,
}

impl<K: Ord, M: Monoid> DynamicSegmentTree<K, M> {
    /// 動的セグ木の初期化
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// 要素の更新
    /// - `key`：更新するキー
    /// - `value`：更新後の値
    pub fn update(&mut self, key: K, value: M::Val) {
        let (new_root, old_key_value) = insert(self.root.take(), key, value);
        self.root = new_root;
        // 要素が追加された場合
        if old_key_value.is_none() {
            self.size += 1;
        }
    }

    /// 要素の削除
    /// - `key`：削除するキー
    pub fn remove(&mut self, key: &K) -> Option<M::Val> {
        let (new_root, old_key_value) = delete(self.root.take(), key);
        self.root = new_root;
        // 削除された要素を返す
        if let Some((_, old_value)) = old_key_value {
            self.size -= 1;
            Some(old_value)
        } else {
            None
        }
    }

    /// 区間の取得
    /// - `[l,r)` の要素を集約する
    pub fn get_range(&self, l: &K, r: &K, begin: &K, end: &K) -> M::Val {
        get_range(&self.root, l, r, begin, end)
    }

    /// 要素数を取得
    pub fn len(&self) -> usize {
        self.size
    }
}
