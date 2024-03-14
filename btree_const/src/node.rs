//! ノード

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub const DEGREE: usize = 4;

/// B木の子ノードの数
pub const CHILDREN: usize = DEGREE;
/// 1つのノードが持つデータの数
pub const CAPACITY: usize = DEGREE - 1;

/// B木のノード
pub type BTreeNode<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

/// 親方向へのポインタ
pub type ParentNode<K, V> = Option<Weak<RefCell<Node<K, V>>>>;

/// B木のノード（内部）
/// ### Generics
/// - `K`：キーの型
/// - `V`：値の型
/// - `DEG`：ノードの持つ子ノードの数の最大値
pub enum Node<K, V> {
    /// 葉ノード
    Leaf {
        /// 親へのポインタ
        parent: ParentNode<K, V>,
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
        parent: ParentNode<K, V>,
        /// キーの配列
        keys: [Option<K>; CAPACITY],
        /// 値の配列
        vals: [Option<V>; CAPACITY],
        /// 子
        children: [BTreeNode<K, V>; CHILDREN],
        /// ノードにあるデータの数
        len: usize,
    },
}

impl<K: Ord, V> Node<K, V> {
    /// 空の葉ノードの作成
    pub fn alloc_leaf() -> BTreeNode<K, V> {
        Some(Rc::new(RefCell::new(Node::Leaf {
            parent: None,
            keys: Default::default(),
            vals: Default::default(),
            len: 0,
        })))
    }

    /// 葉ノードの新規作成
    pub fn alloc_leaf_with_data(key: K, value: V) -> BTreeNode<K, V> {
        // キー配列の初期化
        let mut keys: [Option<K>; CAPACITY] = Default::default();
        keys[0] = Some(key);

        // 値配列の初期化
        let mut vals: [Option<V>; CAPACITY] = Default::default();
        vals[0] = Some(value);

        Some(Rc::new(RefCell::new(Node::Leaf {
            parent: None,
            keys,
            vals,
            len: 1,
        })))
    }

    /// 内部ノードの新規作成
    pub fn alloc_inernal_with_data(key: K, value: V) -> BTreeNode<K, V> {
        // キー配列の初期化
        let mut keys: [Option<K>; CAPACITY] = Default::default();
        keys[0] = Some(key);

        // 値配列の初期化
        let mut vals: [Option<V>; CAPACITY] = Default::default();
        vals[0] = Some(value);

        Some(Rc::new(RefCell::new(Node::Internal {
            parent: None,
            keys,
            vals,
            children: Default::default(),
            len: 1,
        })))
    }

    /// ノードに空きがあるか
    pub fn has_vacant(&self) -> bool {
        match self {
            Node::Internal { len, .. } => *len < CAPACITY,
            Node::Leaf { len, .. } => *len < CAPACITY,
        }
    }
}
