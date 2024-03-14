use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// btreeを作成する
/// ```
/// let tree = btree! {};
/// ```
#[macro_export]
macro_rules! btree {
    ( keys: $keys:expr , vals: $vals:expr , len: $len:expr $(,)* ) => {
        Some(Rc::new(RefCell::new(Node::Leaf {
            parent: None,
            keys: $keys,
            vals: $vals,
            len: $len,
        })))
    };
    ( keys: $keys:expr , vals: $vals:expr , children: $children:expr , len: $len:expr $(,)* ) => {
        Some(Rc::new(RefCell::new(Node::Internal {
            parent: None,
            keys: $keys,
            vals: $vals,
            children: $children,
            len: $len,
        })))
    };
}

pub const CHILDREN: usize = 3;
pub const CAPACITY: usize = 2;

/// B木のノード
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

#[test]
fn test_debug_print() {
    // DEG=3（2-3木）
    let tree = btree! {
        keys: [Some(0), Some(5)],
        vals: [Some("0003"), Some("0005")],
        children: [
            btree! {
                keys: [Some(0), Some(1)],
                vals: [Some("0000"), Some("0001")],
                len: 2,
            },
            btree! {
                keys: [Some(4), None],
                vals: [Some("0004"), None],
                len: 1,
            },
            btree! {
                keys: [Some(6), None],
                vals: [Some("0006"), None],
                len: 1,
            },
        ],
        len: 2,
    };
}
