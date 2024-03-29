//! B木のノード

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// B木のノード
/// ### Generics
/// - `K`：キーの型
/// - `V`：値の型
/// - `DEG`：ノードの持つ子ノードの数の最大値
pub enum Node<K, V, const DEG: usize>
where
    [(); DEG - 1]:,
{
    /// 葉ノード
    Leaf {
        /// 親へのポインタ
        parent: Option<Weak<RefCell<Node<K, V, DEG>>>>,
        /// キーの配列
        keys: [Option<K>; DEG - 1],
        /// 値の配列
        vals: [Option<V>; DEG - 1],
        /// ノードにあるデータの数
        len: usize,
    },
    /// 内部ノード
    Internal {
        /// 親へのポインタ
        parent: Option<Weak<RefCell<Node<K, V, DEG>>>>,
        /// キーの配列
        keys: [Option<K>; DEG - 1],
        /// 値の配列
        vals: [Option<V>; DEG - 1],
        /// 子
        children: [Option<Rc<RefCell<Node<K, V, DEG>>>>; DEG],
        /// ノードにあるデータの数
        len: usize,
    },
}
