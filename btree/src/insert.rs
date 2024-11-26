//! B木にデータを挿入する

use std::borrow::BorrowMut;

use crate::{
    node::{BTreeNode, NodePtr},
    node_util::NodeUtil,
};

/// B木に値を挿入する
/// - `root`：挿入対象の木のルート
/// - `key`：挿入するキー
/// - `value`：挿入する値
pub fn insert<const D: usize, K: Ord, V>(
    root: Option<NodePtr<D, K, V>>,
    key: K,
    value: V,
) -> Option<NodePtr<D, K, V>>
where
    [(); 2 * D - 1]:,
    [Option<K>; 2 * D - 1]: Default,
    [Option<V>; 2 * D - 1]: Default,
{
    let Some(T) = root else {
        // 葉を新規作成する
        return Some(BTreeNode::alloc_leaf_with_data(key, value));
    };

    match *T.borrow_mut() {
        // BTreeNode {
        //     parent,
        //     keys,
        //     vals,
        //     children: Some(children),
        //     size,
        // } => {
        //     todo!()
        // }
        node => {
            // ノードに空きがあるとき
            if !node.is_filled() {
                insert_non_full::<D, _, _>(node, key, value);
            }
            // ノードに空きがないとき
            else {
                todo!()
            }
        }
    }

    Some(T)
}

/// 空きのある葉ノードにデータを挿入する
fn insert_non_full<const D: usize, K, V>(node: &mut BTreeNode<D, K, V>, key: K, value: V)
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    // 後ろにデータを移動し，挿入する位置を見つける
    // insert([1, 3, -], 2)
    // ---
    // 1. [1, 3, -]: idx=2
    // 2. [1, -, 3]: idx=1
    // 3. [1, 2, 3]: idx=1に2を挿入して終了

    // 挿入する位置（末尾）
    let mut idx = 2 * D - 2;

    let keys = &mut node.keys;
    let vals = &mut node.vals;

    keys[idx] = Some(key);
    vals[idx] = Some(value);

    // 正しく整列するまでswap
    while idx > 0 {
        // key以上の値を右に1つずらす
        if keys[idx - 1].is_none() || keys[idx - 1] >= keys[idx] {
            keys.swap(idx - 1, idx);
            vals.swap(idx - 1, idx);
            idx -= 1;
        } else {
            break;
        }
    }

    node.size += 1;
}

/// ノード`x`の`i`番目の子`y`が飽和しているとき，頂点を分割する
///
/// **引数**
/// - `x`：分割する親ノード
/// - `i`：分割する子ノードのインデックス
/// - `y`：分割する子ノード（予め確保する）
fn insert_split_child<const D: usize, K, V, N>(
    x: &mut BTreeNode<D, K, V>,
    i: usize,
    z: &mut BTreeNode<D, K, V>,
) where
    [(); 2 * D - 1]:,
    K: Ord,
{
    let mut y = x.children.unwrap()[i].unwrap();

    let z_keys = &mut z.keys;
    let z_vals = &mut z.vals;

    // キー，値を付け替える
    for j in 0..D - 1 {
        z_keys[j] = y.keys_mut()[j + D].take();
        z_vals[j] = y.vals_mut()[j + D].take();
    }

    // 子を付け替える
    if let Some((mut y_children, mut z_children)) = y.children_mut().as_mut().zip(z.children) {
        for j in 0..D {
            z_children[j] = y_children[j + D].take();
        }
    }

    *y.size_mut() = D - 1;
}
