//! ノード

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::node_util::NodeUtil;

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
    [Option<K>; 2 * D - 1]: Default,
    [Option<V>; 2 * D - 1]: Default,
{
    /// 空の葉ノードの作成
    pub fn alloc_leaf() -> NodePtr<D, K, V> {
        Rc::new(RefCell::new(BTreeNode {
            parent: None,
            keys: Default::default(),
            vals: Default::default(),
            children: None,
            size: 0,
        }))
    }

    /// 葉ノードの新規作成
    pub fn alloc_leaf_with_data(key: K, value: V) -> NodePtr<D, K, V> {
        // キー配列の初期化
        let mut keys: [Option<K>; 2 * D - 1] = Default::default();
        keys[0] = Some(key);

        // 値配列の初期化
        let mut vals: [Option<V>; 2 * D - 1] = Default::default();
        vals[0] = Some(value);

        Rc::new(RefCell::new(BTreeNode {
            parent: None,
            keys,
            vals,
            children: None,
            size: 1,
        }))
    }

    /// 内部ノードの新規作成
    pub fn alloc_internal_with_data(key: K, value: V) -> NodePtr<D, K, V> {
        // キー配列の初期化
        let mut keys: [Option<K>; 2 * D - 1] = Default::default();
        keys[0] = Some(key);

        // 値配列の初期化
        let mut vals: [Option<V>; 2 * D - 1] = Default::default();
        vals[0] = Some(value);

        Rc::new(RefCell::new(BTreeNode {
            parent: None,
            keys,
            vals,
            children: Some(std::array::from_fn(|_| None)),
            size: 1,
        }))
    }

    /// ノードに空きがあるか
    pub fn is_filled(&self) -> bool {
        self.size == 2 * D - 1
    }
}
