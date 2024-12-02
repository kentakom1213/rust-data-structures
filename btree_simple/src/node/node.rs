//! ノード

/// B木のノード
pub type NodePtr<const D: usize, K, V> = Box<BTreeNode<D, K, V>>;

/// ### Generics
/// - `K`：キーの型
/// - `V`：値の型
/// - `DEG`：ノードの持つ子ノードの数の最大値
///
/// ノードは`x`個（`k ≤ x ≤ 2k-1`）のデータをもつ．
/// 更にノードが内部ノードであるとき，`x+1`個の子を持つ．
#[derive(Clone)]
pub struct BTreeNode<const D: usize, K, V>
where
    [(); 2 * D - 1]:,
    [(); 2 * D]:,
{
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
            keys: std::array::from_fn(|_| None),
            vals: std::array::from_fn(|_| None),
            children: None,
            size: 0,
        }
    }

    /// 空の葉ノードを作成し，ポインタを返す
    pub fn alloc_leaf() -> NodePtr<D, K, V> {
        Box::new(BTreeNode::new_leaf())
    }

    /// 空の内部ノードの新規作成
    pub fn new_internal() -> BTreeNode<D, K, V> {
        BTreeNode {
            keys: std::array::from_fn(|_| None),
            vals: std::array::from_fn(|_| None),
            children: Some(std::array::from_fn(|_| None)),
            size: 0,
        }
    }

    /// 空の内部ノードを作成し，ポインタを返す
    pub fn alloc_internal() -> NodePtr<D, K, V> {
        Box::new(BTreeNode::new_internal())
    }

    /// `n`番目のキーを取得する
    pub fn nth_key(&self, n: usize) -> Option<&K> {
        self.keys.get(n).and_then(|x| x.as_ref())
    }

    /// `n`番目の値を取得する
    pub fn nth_val(&self, n: usize) -> Option<&V> {
        self.vals.get(n).and_then(|x| x.as_ref())
    }

    /// `n`番目の値を取得する（可変参照）
    pub fn nth_val_mut(&mut self, n: usize) -> Option<&mut V> {
        self.vals.get_mut(n).and_then(|x| x.as_mut())
    }

    /// `n`番目の子を取得する
    pub fn nth_child(&self, n: usize) -> Option<&NodePtr<D, K, V>> {
        self.children.as_ref()?.get(n).and_then(|x| x.as_ref())
    }

    /// `n`番目の子を取得する（可変参照）
    pub fn nth_child_mut(&mut self, n: usize) -> Option<&mut NodePtr<D, K, V>> {
        self.children.as_mut()?.get_mut(n).and_then(|x| x.as_mut())
    }

    /// `n`番目の子の所有権を取得する
    pub fn take_nth_child(&mut self, n: usize) -> Option<NodePtr<D, K, V>> {
        self.children.as_mut()?.get_mut(n).and_then(|x| x.take())
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
