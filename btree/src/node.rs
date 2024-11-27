//! ノード

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// B木のノード
pub type NodePtr<const D: usize, K, V> = Rc<RefCell<BTreeNode<D, K, V>>>;

/// 親方向へのポインタ
pub type ParentPtr<const D: usize, K, V> = Weak<RefCell<BTreeNode<D, K, V>>>;

/// B木のノード
/// ### Generics
/// - `K`：キーの型
/// - `V`：値の型
/// - `DEG`：ノードの持つ子ノードの数の最大値
///
/// ノードは`x`個（`k ≤ x ≤ 2k-1`）のデータをもつ．
/// 更にノードが内部ノードであるとき，`x+1`個の子を持つ．
pub struct BTreeNode<const D: usize, K, V>
where
    [(); 2 * D - 1]:,
    [(); 2 * D]:,
{
    /// 親へのポインタ
    pub parent: Option<ParentPtr<D, K, V>>,
    /// キーの配列
    pub keys: [Option<K>; 2 * D - 1],
    /// 値の配列
    pub vals: [Option<V>; 2 * D - 1],
    /// 子
    pub children: Option<[Option<NodePtr<D, K, V>>; 2 * D]>,
    /// ノードにあるデータの数
    pub size: usize,
}

impl<const D: usize, K, V> BTreeNode<D, K, V>
where
    [(); 2 * D - 1]:,
    [(); 2 * D]:,
{
    /// 空の葉ノードの新規作成
    pub fn new_leaf() -> BTreeNode<D, K, V> {
        BTreeNode {
            parent: None,
            keys: std::array::from_fn(|_| None),
            vals: std::array::from_fn(|_| None),
            children: None,
            size: 0,
        }
    }

    /// 空の葉ノードを作成し，ポインタを返す
    pub fn alloc_leaf() -> NodePtr<D, K, V> {
        Rc::new(RefCell::new(BTreeNode::new_leaf()))
    }

    /// 空の内部ノードの新規作成
    pub fn new_internal() -> BTreeNode<D, K, V> {
        BTreeNode {
            parent: None,
            keys: std::array::from_fn(|_| None),
            vals: std::array::from_fn(|_| None),
            children: Some(std::array::from_fn(|_| None)),
            size: 0,
        }
    }

    /// 空の内部ノードを作成し，ポインタを返す
    pub fn alloc_internal() -> NodePtr<D, K, V> {
        Rc::new(RefCell::new(BTreeNode::new_internal()))
    }

    /// ノードが葉であるか判定する
    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    /// ノードに空きがあるか
    pub fn is_full(&self) -> bool {
        self.size == 2 * D - 1
    }
}
