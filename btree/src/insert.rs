//! B木にデータを挿入する

use crate::{
    node::{Internal, Leaf, Node, NodePtr},
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
        return Some(Node::alloc_leaf_with_data(key, value));
    };

    match &mut *T.borrow_mut() {
        Node::Internal(Internal {
            parent,
            keys,
            vals,
            children,
            size,
        }) => {
            todo!()
        }
        Node::Leaf(node) => {
            // ノードに空きがあるとき
            if !node.is_filled() {
                insert_non_full::<D, _, _, _>(node, key, value);
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
fn insert_non_full<const D: usize, K, V, N>(node: &mut N, key: K, value: V)
where
    [(); 2 * D - 1]:,
    K: Ord,
    N: NodeUtil<D, K, V>,
{
    // 後ろにデータを移動し，挿入する位置を見つける
    // insert([1, 3, -], 2)
    // ---
    // 1. [1, 3, -]: idx=2
    // 2. [1, -, 3]: idx=1
    // 3. [1, 2, 3]: idx=1に2を挿入して終了

    // 挿入する位置（末尾）
    let mut idx = 2 * D - 2;

    let (keys, vals) = node.keys_and_vals_mut();

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

    *node.size_mut() += 1;
}

/// 空きのない葉ノードにデータを挿入する
fn insert_split_child<const D: usize, K, V, N>(node: &mut N, key: K, value: V)
where
    [(); 2 * D - 1]:,
    K: Ord,
    N: NodeUtil<D, K, V>,
{
    todo!()
}
