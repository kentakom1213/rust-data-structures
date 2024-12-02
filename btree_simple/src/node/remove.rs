//! B木からデータを削除する

use std::fmt::Debug;

use crate::{BTreeNode, NodePtr, NodeUtil};

/// 削除するキーを指定するための列挙型
#[derive(Debug)]
pub enum RemoveKey<'a, K: Ord> {
    /// 最小値を削除
    Min,
    /// 指定したキーを削除
    Key(&'a K),
    /// 最大値を削除
    Max,
}

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
    key: RemoveKey<K>,
) -> (Option<NodePtr<D, K, V>>, Option<(K, V)>)
where
    [(); 2 * D - 1]:,
    K: Ord + Debug,
    V: Debug,
{
    let Some(mut node) = root else {
        return (None, None);
    };

    // 葉である場合
    if node.is_leaf() {
        let (removed_key_value, _) = remove_from_sufficient_node(&mut *node.as_mut(), key);

        return (Some(node), removed_key_value);
    }

    let size = *node.size();

    for i in 0..=size {
        if match key {
            RemoveKey::Min => i == 0,
            RemoveKey::Key(k) => i == size || k < node.nth_key(i).unwrap(),
            RemoveKey::Max => i == size,
        } {
            // i番目の子から削除する
            let ch_size = *node.nth_child(i).unwrap().size();

            if ch_size >= D {
                // 子のサイズがD以上の場合，再帰的に削除する
                let (ch, removed_key_value) = remove(node.take_nth_child(i), key);

                node.children.as_mut().unwrap()[i] = ch;

                return (Some(node), removed_key_value);
            }

            let lch_size = if i == 0 {
                0
            } else {
                *node.nth_child(i - 1).unwrap().size()
            };
            let rch_size = if i == size {
                0
            } else {
                *node.nth_child(i + 1).unwrap().size()
            };

            if lch_size >= D {
                // 左の子がD以上の場合，左の子から最大値を削除し，その値をi番目のキーに移植する
                let (max_key_value, rightmost_child) = remove_from_sufficient_node(
                    node.nth_child_mut(i - 1).as_mut().unwrap(),
                    RemoveKey::Max,
                );

                let mut ch = node.take_nth_child(i);
                let ch_size = *ch.as_ref().unwrap().size();

                // 右に1つづつずらす
                if let Some(ch_children) = ch.as_mut().unwrap().children.as_mut() {
                    ch_children.swap(ch_size, ch_size + 1);
                }

                for j in (0..ch_size).rev() {
                    ch.as_mut().unwrap().keys.swap(j, j + 1);
                    ch.as_mut().unwrap().vals.swap(j, j + 1);
                    if let Some(ch_children) = ch.as_mut().unwrap().children.as_mut() {
                        ch_children.swap(j, j + 1);
                    }
                }

                // i番目のキーをchに追加
                ch.as_mut().unwrap().keys[0] = node.keys[i - 1].take();
                ch.as_mut().unwrap().vals[0] = node.vals[i - 1].take();

                ch.as_mut().unwrap().size += 1;

                if let Some((k, v)) = max_key_value {
                    node.keys[i - 1] = Some(k);
                    node.vals[i - 1] = Some(v);
                }

                if let Some(ch_children) = ch.as_mut().unwrap().children.as_mut() {
                    ch_children[0] = rightmost_child;
                }

                // i番目の子から削除する
                let (ch, removed_key_value) = remove(ch, key);

                node.children.as_mut().unwrap()[i] = ch;

                return (Some(node), removed_key_value);
            } else if rch_size >= D {
                // 右の子がD以上の場合，右の子から最小値を削除し，その値をi番目のキーに移植する
                let (min_key_value, leftmost_child) = remove_from_sufficient_node(
                    node.nth_child_mut(i + 1).as_mut().unwrap(),
                    RemoveKey::Min,
                );

                let mut ch = node.take_nth_child(i);

                // i - 1番目のキーをchに追加
                ch.as_mut().unwrap().keys[ch_size] = node.keys[i].take();
                ch.as_mut().unwrap().vals[ch_size] = node.vals[i].take();

                if let Some((k, v)) = min_key_value {
                    node.keys[i] = Some(k);
                    node.vals[i] = Some(v);
                }

                if let Some((ch_children, leftmost_child)) =
                    ch.as_mut().unwrap().children.as_mut().zip(leftmost_child)
                {
                    ch_children[ch_size + 1] = Some(leftmost_child);
                }

                ch.as_mut().unwrap().size += 1;

                // i番目の子から削除する
                let (ch, removed_key_value) = remove(ch, key);

                node.children.as_mut().unwrap()[i] = ch;

                return (Some(node), removed_key_value);
            } else {
                // 左右の子をマージする
                node = if i == size {
                    merge_childs(Some(node), i - 1).unwrap()
                } else {
                    merge_childs(Some(node), i).unwrap()
                };

                // i番目の子から削除する
                let (new_node, removed_key_value) = remove(Some(node), key);

                return (new_node, removed_key_value);
            }
        }

        if i < size && matches!(key, RemoveKey::Key(k) if k == node.nth_key(i).unwrap()) {
            // i番目の値を削除する
            let removed_key_value;

            let lch_size = *node.nth_child(i).unwrap().size();
            let rch_size = *node.nth_child(i + 1).unwrap().size();

            if lch_size >= D {
                // 左の子がD以上の場合，左の子から右側に移動させる
                let (lch, max_key_value) = remove(node.take_nth_child(i), RemoveKey::Max);

                // i番目の値を削除する
                removed_key_value = node.keys[i].take().zip(node.vals[i].take());

                // 左の子の最大値で置き換え
                if let Some((k, v)) = max_key_value {
                    node.keys[i] = Some(k);
                    node.vals[i] = Some(v);
                }

                node.children.as_mut().unwrap()[i] = lch;
            } else if rch_size >= D {
                // 右の子がD以上の場合，右の子から最小値を削除する
                let (rch, min_key_value) = remove(node.take_nth_child(i + 1), RemoveKey::Min);

                // i番目の値を削除する
                removed_key_value = node.keys[i].take().zip(node.vals[i].take());

                // 右の子の最小値で置き換え
                if let Some((k, v)) = min_key_value {
                    node.keys[i] = Some(k);
                    node.vals[i] = Some(v);
                }

                node.children.as_mut().unwrap()[i + 1] = rch;
            } else {
                // 左右の子をマージする
                node = merge_childs(Some(node), i).unwrap();

                // i番目の子から削除する
                let new_node;
                (new_node, removed_key_value) = remove(Some(node), key);

                node = new_node.unwrap();
            }

            return (Some(node), removed_key_value);
        }
    }

    unreachable!()
}

/// サイズが`D`以上のノードからキー`key`を削除する
///
/// **引数**
/// - `root`：削除対象の木のルート
/// - `key`：削除するキー
///
/// **戻り値**
/// - `Option<(K, V)>`：削除されたキーと値
/// - `Option<NodePtr<D, K, V>>`：（Min/Maxのとき）端の子
fn remove_from_sufficient_node<const D: usize, K, V>(
    node: &mut BTreeNode<D, K, V>,
    key: RemoveKey<K>,
) -> (Option<(K, V)>, Option<NodePtr<D, K, V>>)
where
    [(); 2 * D - 1]:,
    K: Ord + Debug,
    V: Debug,
{
    debug_assert!(node.is_leaf() || node.size >= D);

    let mut removed_key = None;
    let mut removed_val = None;
    let mut edge_child = None;

    for i in 0..node.size {
        if removed_key.is_some() {
            // 削除済の場合，左に1つづつずらす
            node.keys[i - 1] = node.keys[i].take();
            node.vals[i - 1] = node.vals[i].take();
        } else if i < node.size
            && match key {
                RemoveKey::Min => i == 0,
                RemoveKey::Key(k) => node.keys[i].as_ref().unwrap() == k,
                RemoveKey::Max => i == node.size - 1,
            }
        {
            // 値が一致する場合，削除
            removed_val = node.vals[i].take();
            removed_key = node.keys[i].take();
        } else if match key {
            RemoveKey::Min => true,
            RemoveKey::Key(k) => node.keys[i].as_ref().unwrap() > k,
            RemoveKey::Max => false,
        } {
            break;
        }
    }

    if !node.is_leaf() && removed_key.is_some() {
        edge_child = match key {
            RemoveKey::Min => {
                let leftmost = node.take_nth_child(0);

                // 1つづつ左にずらす
                for i in 1..=node.size {
                    node.children.as_mut().unwrap().swap(i - 1, i);
                }

                leftmost
            }
            RemoveKey::Key(_) => None,
            RemoveKey::Max => node.take_nth_child(node.size),
        };
    }

    if removed_key.is_some() {
        node.size -= 1;
    }

    (removed_key.zip(removed_val), edge_child)
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
    lch.vals[lch_size] = node.vals[i].take();
    lch.keys[lch_size] = node.keys[i].take();

    // 親のキー，値，子へのポインタを1つずつ左にずらす
    for j in i + 1..2 * D - 1 {
        node.keys[j - 1] = node.keys[j].take();
        node.vals[j - 1] = node.vals[j].take();
        node.children.as_mut().unwrap().swap(j, j + 1);
    }

    node.size -= 1;

    // 右の子の値を左の子に移植
    let mut j = lch_size + 1;
    let rch_size = *rch.size();

    for k in 0..rch_size {
        lch.keys[j] = rch.keys[k].take();
        lch.vals[j] = rch.vals[k].take();
        j += 1;
    }

    // 内部ノードの場合は右の子の子も移植
    if let Some((lch_ch, rch_ch)) = lch.children.as_mut().zip(rch.children.as_mut()) {
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
    use crate::{btree, print_as_tree, BTreeNode, NodePtr};

    use super::*;

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
            let (res, ptr) =
                remove_from_sufficient_node(tree.as_mut().unwrap(), RemoveKey::Key(&c));

            print_as_tree(&tree);
            println!("removed (key, val) = {:?}", res);

            assert!(ptr.is_none());

            if let Some((k, _)) = res {
                assert_eq!(k, c);
            }
        }
    }

    #[test]
    fn test_remove_min_max() {
        let mut tree: Option<NodePtr<4, char, i32>> = btree! {
            keys: [Some('a'), Some('c'), Some('e'), Some('g'), Some('i'), Some('k'), None],
            vals: [Some(434), Some(112), Some(605), Some(705), Some(334), Some(963), None],
            size: 6
        };

        print_as_tree(&tree);

        println!("> remove min");
        let (min, ptr) = remove_from_sufficient_node(tree.as_mut().unwrap(), RemoveKey::Min);
        print_as_tree(&tree);

        assert!(ptr.is_none());
        assert!(min.is_some());
        assert_eq!(min.unwrap(), ('a', 434));

        println!("> remove max");
        let (max, ptr) = remove_from_sufficient_node(tree.as_mut().unwrap(), RemoveKey::Max);
        print_as_tree(&tree);

        assert!(ptr.is_none());
        assert!(max.is_some());
        assert_eq!(max.unwrap(), ('k', 963));
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

    #[test]
    fn test_remove_min_max_2() {
        let mut tree = build_tree_2();

        print_as_tree(&tree);

        println!("> remove min");
        let min;
        (tree, min) = remove(tree, RemoveKey::Min);

        print_as_tree(&tree);
        println!("removed (key, val) = {:?}", min);

        assert!(min.is_some());
        assert_eq!(min.unwrap(), ('a', "Apple".to_string()));

        println!("> remove max");
        let max;
        (tree, max) = remove(tree, RemoveKey::Max);

        print_as_tree(&tree);
        println!("removed (key, val) = {:?}", max);

        assert!(max.is_some());
        assert_eq!(max.unwrap(), ('g', "Grape".to_string()));

        println!("> remove min");
        let min;
        (tree, min) = remove(tree, RemoveKey::Min);

        print_as_tree(&tree);
        println!("removed (key, val) = {:?}", min);
        assert_eq!(min.unwrap(), ('b', "Banana".to_string()));

        println!("> remove min");
        let min;
        (tree, min) = remove(tree, RemoveKey::Min);

        print_as_tree(&tree);
        println!("removed (key, val) = {:?}", min);
        assert_eq!(min.unwrap(), ('c', "Cherry".to_string()));

        println!("> remove min");
        let min;
        (tree, min) = remove(tree, RemoveKey::Min);

        print_as_tree(&tree);
        println!("removed (key, val) = {:?}", min);
        assert_eq!(min.unwrap(), ('d', "Doughnut".to_string()));

        println!("> remove min");
        let min;
        (tree, min) = remove(tree, RemoveKey::Min);

        print_as_tree(&tree);
        println!("removed (key, val) = {:?}", min);
        assert_eq!(min.unwrap(), ('e', "Eggplant".to_string()));

        println!("> remove max");
        let max;
        (tree, max) = remove(tree, RemoveKey::Max);

        print_as_tree(&tree);
        println!("removed (key, val) = {:?}", max);
        assert_eq!(max.unwrap(), ('f', "Fruit".to_string()));
    }
}
