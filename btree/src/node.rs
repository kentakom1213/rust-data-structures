//! B木のノード

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// B木のノード
pub enum Node<K, V, const CAP: usize, const DEG: usize> {
    /// 葉ノード
    Leaf {
        /// 親へのポインタ
        parent: Option<Weak<RefCell<Node<K, V, CAP, DEG>>>>,
        /// キーの配列
        keys: [K; CAP],
        /// 値の配列
        vals: [V; CAP],
    },
    /// 内部ノード
    Internal {
        /// 親へのポインタ
        parent: Option<Weak<RefCell<Node<K, V, CAP, DEG>>>>,
        /// キーの配列
        keys: [K; CAP],
        /// 値の配列
        vals: [V; CAP],
        /// 子
        children: [Option<Rc<RefCell<Node<K, V, CAP, DEG>>>>; DEG],
    },
}
