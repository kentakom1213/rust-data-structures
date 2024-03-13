//! B木のノード

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// B木のノード
/// - `DEG`：ノードの持つデータの数
/// - `DEG`：子ノードの数
///
pub enum Node<K, V, const DEG: usize>
where
    [(); DEG - 1]:,
{
    /// 葉ノード
    Leaf {
        /// 親へのポインタ
        parent: Option<Weak<RefCell<Node<K, V, DEG>>>>,
        /// キーの配列
        keys: [K; DEG - 1],
        /// 値の配列
        vals: [V; DEG - 1],
    },
    /// 内部ノード
    Internal {
        /// 親へのポインタ
        parent: Option<Weak<RefCell<Node<K, V, DEG>>>>,
        /// キーの配列
        keys: [K; DEG - 1],
        /// 値の配列
        vals: [V; DEG - 1],
        /// 子
        children: [Option<Rc<RefCell<Node<K, V, DEG>>>>; DEG],
    },
}
