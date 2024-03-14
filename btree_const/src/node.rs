//! ノード

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub const CHILDREN: usize = 3;
pub const CAPACITY: usize = 2;

/// B木のノード
pub type BTreeNode<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

/// B木のノード（内部）
/// ### Generics
/// - `K`：キーの型
/// - `V`：値の型
/// - `DEG`：ノードの持つ子ノードの数の最大値
pub enum Node<K, V> {
    /// 葉ノード
    Leaf {
        /// 親へのポインタ
        parent: Option<Weak<RefCell<Node<K, V>>>>,
        /// キーの配列
        keys: [Option<K>; CAPACITY],
        /// 値の配列
        vals: [Option<V>; CAPACITY],
        /// ノードにあるデータの数
        len: usize,
    },
    /// 内部ノード
    Internal {
        /// 親へのポインタ
        parent: Option<Weak<RefCell<Node<K, V>>>>,
        /// キーの配列
        keys: [Option<K>; CAPACITY],
        /// 値の配列
        vals: [Option<V>; CAPACITY],
        /// 子
        children: [Option<Rc<RefCell<Node<K, V>>>>; CHILDREN],
        /// ノードにあるデータの数
        len: usize,
    },
}
