//! 辞書型の実装

use std::fmt::Debug;

use crate::{node::*, print_util::pretty_print};

pub struct AATreeMap<K: Ord, V> {
    pub root: AATreeNode<K, V>,
}

impl<K: Ord, V> AATreeMap<K, V> {
    /// mapの初期化
    pub fn new() -> Self {
        Self { root: None }
    }

    /// キーに対応する値の参照を取得する
    pub fn get(&self, key: &K) -> Option<&V> {
        get(&self.root, key)
    }

    /// キーに対応する値の**可変**参照を取得する
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        get_mut(&mut self.root, key)
    }

    /// キーに対応する値を挿入する
    pub fn insert(&mut self, key: K, value: V) {
        self.root = insert(self.root.take(), key, value);
    }

    /// キーに対応する値を削除する
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let (new_root, old) = delete(self.root.take(), key);
        self.root = new_root;
        old.map(|old| old.1)
    }
}

impl<K: Ord + Debug, V: Debug> AATreeMap<K, V> {
    /// 整形して表示する
    pub fn pretty_print(&self) {
        pretty_print(&self.root);
    }
}
