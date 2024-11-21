//! B木にデータを挿入する

use crate::node::{Node, NodePtr, CAPACITY};

/// B木に値を挿入する
/// - `root`：挿入対象の木のルート
/// - `key`：挿入するキー
/// - `value`：挿入する値
pub fn insert<K: Ord, V>(root: Option<NodePtr<K, V>>, key: K, value: V) -> Option<NodePtr<K, V>> {
    let Some(T) = root else {
        // 葉を新規作成する
        return Some(Node::alloc_leaf_with_data(key, value));
    };

    match &mut *T.borrow_mut() {
        Node::Internal {
            parent,
            keys,
            vals,
            children,
            size,
        } => {
            todo!()
        }
        Node::Leaf {
            parent,
            keys,
            vals,
            size,
        } => {
            // ノードに空きがあるとき
            if *size < CAPACITY {
                insert_leaf_with_vacent(keys, vals, key, value);
                *size += 1;
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
fn insert_leaf_with_vacent<K: Ord, V>(
    keys: &mut [Option<K>],
    vals: &mut [Option<V>],
    key: K,
    value: V,
) {
    // 後ろにデータを移動し，挿入する位置を見つける
    // insert([1, 3, -], 2)
    // ---
    // 1. [1, 3, -]: idx=2
    // 2. [1, -, 3]: idx=1
    // 3. [1, 2, 3]: idx=1に2を挿入して終了

    // 挿入する位置（末尾）
    let mut idx = CAPACITY - 1;

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
}
