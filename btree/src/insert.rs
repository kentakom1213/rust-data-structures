//! B木にデータを挿入する

use crate::node::{Internal, Leaf, Node, NodePtr};

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
    [(); D + 1]:,
    [Option<K>; D]: Default,
    [Option<V>; D]: Default,
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
            if node.size < D {
                insert_leaf_with_vacent::<D, _, _>(node, key, value);
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
fn insert_leaf_with_vacent<const D: usize, K: Ord, V>(node: &mut Leaf<D, K, V>, key: K, value: V)
where
    [(); D + 1]:,
{
    // 後ろにデータを移動し，挿入する位置を見つける
    // insert([1, 3, -], 2)
    // ---
    // 1. [1, 3, -]: idx=2
    // 2. [1, -, 3]: idx=1
    // 3. [1, 2, 3]: idx=1に2を挿入して終了

    // 挿入する位置（末尾）
    let mut idx = D - 1;

    node.keys[idx] = Some(key);
    node.vals[idx] = Some(value);

    // 正しく整列するまでswap
    while idx > 0 {
        // key以上の値を右に1つずらす
        if node.keys[idx - 1].is_none() || node.keys[idx - 1] >= node.keys[idx] {
            node.keys.swap(idx - 1, idx);
            node.vals.swap(idx - 1, idx);
            idx -= 1;
        } else {
            break;
        }
    }

    node.size += 1;
}
