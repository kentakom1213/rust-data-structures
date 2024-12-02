//! B木からデータを削除する

use crate::{
    node::{BTreeNode, NodePtr},
    node_util::NodeUtil,
    search::{max_key, min_key},
};

/// B木から値を削除する．複数の値が存在する場合，そのうち一つのキーとそれに対応する値を削除する．
///
/// **引数**
/// - `root`：削除対象の木のルート
/// - `key`：削除するキー
///
/// **戻り値**
/// - `Option<NodePtr<D, K, V>>`：削除後の木のルート
/// - `Option<(K, V)>)`：削除されたキーと値
pub fn remove<const D: usize, K, V>(
    root: Option<NodePtr<D, K, V>>,
    key: &K,
) -> (Option<NodePtr<D, K, V>>, Option<(K, V)>)
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    let Some(mut node) = root else {
        return (None, None);
    };

    // 葉である場合
    if node.is_leaf() {
        let removed_key_value = remove_from_leaf(&mut *node.as_mut(), key);

        return (Some(node), removed_key_value);
    }

    let size = *node.size();

    for i in 0..size {
        if key < node.nth_key(i).unwrap() {
            // i番目の子から削除する
            todo!()
        }
        if key == node.nth_key(i).unwrap() {
            // i番目の値を削除する
            let removed_key = node.keys[i].take();
            let removed_val = node.vals[i].take();

            let lch_size = *node.nth_child(i).unwrap().size();
            let rch_size = *node.nth_child(i + 1).unwrap().size();

            if lch_size >= D {
                // 左の子がD以上の場合，左の子から最大値を削除する
                todo!()
            } else if rch_size >= D {
                // 右の子がD以上の場合，右の子から最小値を削除する
                todo!()
            } else {
                // 左右の子をマージする
                todo!()
            }

            return (Some(node), removed_key.zip(removed_val));
        }
    }

    todo!()
}

/// 葉からキー`key`を削除する
/// - `root`：削除対象の木のルート
/// - `key`：削除するキー
fn remove_from_leaf<const D: usize, K, V>(leaf: &mut BTreeNode<D, K, V>, key: &K) -> Option<(K, V)>
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    debug_assert!(leaf.is_leaf());

    let mut removed_key = None;
    let mut removed_val = None;

    for i in 0..leaf.size {
        if removed_key.is_some() {
            // 削除済の場合，左に1つづつずらす
            leaf.keys[i - 1] = leaf.keys[i].take();
            leaf.vals[i - 1] = leaf.vals[i].take();
        } else if leaf.keys[i].as_ref().unwrap() == key {
            // 値が一致する場合，削除
            removed_key = leaf.keys[i].take();
            removed_val = leaf.vals[i].take();
        } else if leaf.keys[i].as_ref().unwrap() > key {
            break;
        }
    }

    if removed_key.is_some() {
        leaf.size -= 1;
    }

    removed_key.zip(removed_val)
}

/// ノード`node`の`i`番目の子と`i+1`番目の子をマージする
///
/// **引数**
/// - `node`：分割する親ノード
/// - `i`：マージする子の左側のインデックス
fn merge_childs<const D: usize, K, V>(
    node: Option<NodePtr<D, K, V>>,
    i: usize,
) -> Option<NodePtr<D, K, V>>
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    let mut node = node?;

    debug_assert!(!node.is_leaf());
    debug_assert!(node.nth_child(i).is_some_and(|x| *x.size() <= D - 1));
    debug_assert!(node.nth_child(i + 1).is_some_and(|x| *x.size() <= D - 1));

    let mut lch = node.take_nth_child(i).unwrap();
    let mut rch = node.take_nth_child(i + 1).unwrap();

    // 親のi番目の値を左の子に移植
    let lch_size = *lch.size();
    lch.vals_mut()[lch_size] = node.vals[i].take();
    lch.keys_mut()[lch_size] = node.keys[i].take();

    // 親のキー，値，子へのポインタを1つずつ左にずらす
    for j in i + 1..node.size {
        node.keys[j - 1] = node.keys[j].take();
        node.vals[j - 1] = node.vals[j].take();
        node.children.as_mut().unwrap().swap(j, j + 1);
    }
    node.children
        .as_mut()
        .unwrap()
        .swap(node.size, node.size + 1);

    node.size -= 1;

    // 右の子の値を左の子に移植
    let mut j = lch_size + 1;
    let rch_size = *rch.size();

    for k in 0..rch_size {
        lch.keys_mut()[j] = rch.keys_mut()[k].take();
        lch.vals_mut()[j] = rch.vals_mut()[k].take();
        j += 1;
    }

    // 内部ノードの場合は右の子の子も移植
    if let Some((lch_ch, rch_ch)) = lch.children_mut().as_mut().zip(rch.children_mut().as_mut()) {
        let mut j = lch_size + 1;

        for k in 0..rch_size {
            lch_ch[j] = rch_ch[k].take();
            j += 1;
        }

        lch_ch[j] = rch_ch[rch_size].take();
    }

    *lch.size_mut() = lch_size + 1 + rch_size;

    if node.size > 0 {
        // 左のノードを再度つける
        node.children.as_mut().unwrap()[i].replace(lch);

        Some(node)
    } else {
        // ルートノードが空になった場合，左の子を新しいルートとする
        Some(lch)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        btree,
        debug::print_as_tree,
        node::{BTreeNode, NodePtr},
    };

    use super::{merge_childs, remove_from_leaf};

    #[test]
    fn test_remove_from_leaf() {
        let mut tree: Option<NodePtr<4, char, i32>> = btree! {
            keys: [Some('a'), Some('c'), Some('e'), Some('g'), Some('i'), Some('k'), None],
            vals: [Some(434), Some(112), Some(605), Some(705), Some(334), Some(963), None],
            size: 6
        };

        print_as_tree(&tree);

        for c in ['g', 'x', 'i', 'a', 'c', 'e', 'a', 'k'] {
            println!("> remove \'{c}\'");
            let res = remove_from_leaf(tree.as_mut().unwrap(), &c);

            print_as_tree(&tree);
            println!("removed (key, val) = {:?}", res);

            if let Some((k, _)) = res {
                assert_eq!(k, c);
            }
        }
    }

    fn build_tree_1() -> Option<NodePtr<3, char, String>> {
        btree! {
            keys: [Some('b'), Some('e'), Some('g'), None, None],
            vals: [Some("Bob".to_string()), Some("Emily".to_string()), Some("Grace".to_string()), None, None],
            children: [
                btree! {
                    keys: [Some('a'), None, None, None, None],
                    vals: [Some("Alice".to_string()), None, None, None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some('c'), Some('d'), None, None, None],
                    vals: [Some("Charlie".to_string()), Some("David".to_string()), None, None, None],
                    size: 2,
                },
                btree! {
                    keys: [Some('f'), None, None, None, None],
                    vals: [Some("Frank".to_string()), None, None, None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some('h'), None, None, None, None],
                    vals: [Some("Helen".to_string()), None, None, None, None],
                    size: 1,
                },
                None,
                None,
            ],
            size: 3
        }
    }

    #[test]
    fn test_merge_childs() {
        println!("> merge at 0");
        {
            let mut tree = build_tree_1();

            print_as_tree(&tree);

            tree = merge_childs(tree, 0);

            print_as_tree(&tree);
        }

        println!("> merge at 1");
        {
            let mut tree = build_tree_1();

            print_as_tree(&tree);

            tree = merge_childs(tree, 1);

            print_as_tree(&tree);
        }

        println!("> merge at 2");
        {
            let mut tree = build_tree_1();

            print_as_tree(&tree);

            tree = merge_childs(tree, 2);

            print_as_tree(&tree);
        }
    }

    fn build_tree_2() -> Option<NodePtr<2, char, String>> {
        btree! {
            keys: [Some('d'), None, None],
            vals: [Some("Doughnut".to_string()), None, None],
            children: [
                btree! {
                    keys: [Some('b'), None, None],
                    vals: [Some("Banana".to_string()), None, None],
                    children: [
                        btree! {
                            keys: [Some('a'), None, None],
                            vals: [Some("Apple".to_string()), None, None],
                            size: 1,
                        },
                        btree! {
                            keys: [Some('c'), None, None],
                            vals: [Some("Cherry".to_string()), None, None],
                            size: 1,
                        },
                        None,
                        None,
                    ],
                    size: 1,
                },
                btree! {
                    keys: [Some('f'), None, None],
                    vals: [Some("Fruit".to_string()), None, None],
                    children: [
                        btree! {
                            keys: [Some('e'), None, None],
                            vals: [Some("Eggplant".to_string()), None, None],
                            size: 1,
                        },
                        btree! {
                            keys: [Some('g'), None, None],
                            vals: [Some("Grape".to_string()), None, None],
                            size: 1,
                        },
                        None,
                        None,
                    ],
                    size: 1,
                },
                None,
                None,
            ],
            size: 1
        }
    }

    #[test]
    fn test_merge_childs_2() {
        println!("> merge at 0");
        {
            let mut tree = build_tree_2();

            print_as_tree(&tree);

            tree = merge_childs(tree, 0);

            print_as_tree(&tree);
        }
    }
}
