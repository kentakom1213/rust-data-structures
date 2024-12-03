//! 検索を行う

use super::NodePtr;

/// B木のノードからキーを検索する．
///
/// **引数**
/// - `root`: 検索を行うノードのポインタ
/// - `key`: 検索するキー
///
/// **戻り値**
/// - `Some(&V)`: キーが見つかった場合，そのキーに対応する値への参照
pub fn get<'a, const D: usize, K, V>(root: &'a Option<NodePtr<D, K, V>>, key: &K) -> Option<&'a V>
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    let mut node = root.as_ref()?;

    'outer: while !node.is_leaf() {
        for i in 0..node.size {
            if key < node.keys[i].as_ref().unwrap() {
                node = node.children.as_ref().unwrap()[i].as_ref().unwrap();
                continue 'outer;
            }
            if key == node.keys[i].as_ref().unwrap() {
                return node.vals[i].as_ref();
            }
        }
        // 右端の子に移動
        node = node.children.as_ref().unwrap()[node.size].as_ref().unwrap();
    }

    // 葉ノードの検索
    for i in 0..node.size {
        if key == node.keys[i].as_ref().unwrap() {
            return node.vals[i].as_ref();
        }
    }

    None
}

/// B木のノードからキーを検索し，可変参照を取得する．
///
/// **引数**
/// - `root`: 検索を行うノードのポインタ
/// - `key`: 検索するキー
///
/// **戻り値**
/// - `Some(&V)`: キーが見つかった場合，そのキーに対応する値への可変参照
pub fn get_mut<'a, const D: usize, K, V>(
    root: &'a mut Option<NodePtr<D, K, V>>,
    key: &K,
) -> Option<&'a mut V>
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    let mut node = root.as_mut()?;

    'outer: while !node.is_leaf() {
        for i in 0..node.size {
            if key < node.keys[i].as_ref().unwrap() {
                node = node.children.as_mut().unwrap()[i].as_mut().unwrap();
                continue 'outer;
            }
            if key == node.keys[i].as_ref().unwrap() {
                return node.vals[i].as_mut();
            }
        }
        // 右端の子に移動
        node = node.children.as_mut().unwrap()[node.size].as_mut().unwrap();
    }

    // 葉ノードの検索
    for i in 0..node.size {
        if key == node.keys[i].as_ref().unwrap() {
            return node.vals[i].as_mut();
        }
    }

    None
}
