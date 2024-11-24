//! ノード

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// B木のノード
pub type NodePtr<const D: usize, K, V> = Rc<RefCell<Node<D, K, V>>>;

/// 親方向へのポインタ
pub type ParentPtr<const D: usize, K, V> = Weak<RefCell<Node<D, K, V>>>;

/// 葉ノード
pub struct Leaf<const D: usize, K, V>
where
    [(); D + 1]:,
{
    /// 親へのポインタ
    pub parent: Option<ParentPtr<D, K, V>>,
    /// キーの配列
    pub keys: [Option<K>; D],
    /// 値の配列
    pub vals: [Option<V>; D],
    /// ノードにあるデータの数
    pub size: usize,
}

/// 内部ノード
pub struct Internal<const D: usize, K, V>
where
    [(); D + 1]:,
{
    /// 親へのポインタ
    pub parent: Option<ParentPtr<D, K, V>>,
    /// キーの配列
    pub keys: [Option<K>; D],
    /// 値の配列
    pub vals: [Option<V>; D],
    /// 子
    pub children: [Option<NodePtr<D, K, V>>; D + 1],
    /// ノードにあるデータの数
    pub size: usize,
}

/// B木のノード
/// ### Generics
/// - `K`：キーの型
/// - `V`：値の型
/// - `DEG`：ノードの持つ子ノードの数の最大値
pub enum Node<const D: usize, K, V>
where
    [(); D + 1]:,
{
    /// 葉ノード
    Leaf(Leaf<D, K, V>),
    /// 内部ノード
    Internal(Internal<D, K, V>),
}

impl<const D: usize, K, V> Node<D, K, V>
where
    [(); D + 1]:,
    [Option<K>; D]: Default,
    [Option<V>; D]: Default,
{
    /// 空の葉ノードの作成
    pub fn alloc_leaf() -> NodePtr<D, K, V> {
        Rc::new(RefCell::new(Node::Leaf(Leaf {
            parent: None,
            keys: Default::default(),
            vals: Default::default(),
            size: 0,
        })))
    }

    /// 葉ノードの新規作成
    pub fn alloc_leaf_with_data(key: K, value: V) -> NodePtr<D, K, V> {
        // キー配列の初期化
        let mut keys: [Option<K>; D] = Default::default();
        keys[0] = Some(key);

        // 値配列の初期化
        let mut vals: [Option<V>; D] = Default::default();
        vals[0] = Some(value);

        Rc::new(RefCell::new(Node::Leaf(Leaf {
            parent: None,
            keys,
            vals,
            size: 1,
        })))
    }

    /// 内部ノードの新規作成
    pub fn alloc_inernal_with_data(key: K, value: V) -> NodePtr<D, K, V> {
        // キー配列の初期化
        let mut keys: [Option<K>; D] = Default::default();
        keys[0] = Some(key);

        // 値配列の初期化
        let mut vals: [Option<V>; D] = Default::default();
        vals[0] = Some(value);

        Rc::new(RefCell::new(Node::Internal(Internal {
            parent: None,
            keys,
            vals,
            children: std::array::from_fn(|_| None),
            size: 1,
        })))
    }

    /// ノードに空きがあるか
    pub fn has_vacant(&self) -> bool {
        match self {
            Node::Internal(Internal { size, .. }) => *size < D,
            Node::Leaf(Leaf { size, .. }) => *size < D,
        }
    }
}
