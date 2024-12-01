//! B木からデータを削除する

use crate::node::{BTreeNode, NodePtr};

/// B木から値を削除する．複数の値が存在する場合，そのうち一つのキーとそれに対応する値を削除する．
/// - `root`：削除対象の木のルート
/// - `key`：削除するキー
pub fn remove<const D: usize, K, V>(
    root: Option<NodePtr<D, K, V>>,
    _key: K,
) -> Option<NodePtr<D, K, V>>
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    let Some(_root) = root else {
        return None;
    };

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
    assert!(leaf.is_leaf());

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

/// ノード`x`の`i`番目の子`y`が飽和しているとき，頂点を分割する
///
/// **引数**
/// - `x`：分割する親ノード
/// - `i`：分割する子ノードのインデックス
fn merge_child<const D: usize, K, V>(x: &mut BTreeNode<D, K, V>, _i: usize)
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    assert!(!x.is_leaf());
    // assert!(!x.is_full());
    // assert!(x.children.as_ref().unwrap()[i].is_some());

    // let x_children = x.children.as_mut().unwrap();

    // let mut y = x_children[i].clone().unwrap();

    // let mut z = if y.is_leaf() {
    //     BTreeNode::new_leaf()
    // } else {
    //     BTreeNode::new_internal()
    // };

    // // キー，値を付け替える
    // for j in 0..D - 1 {
    //     z.keys[j] = y.keys_mut()[j + D].take();
    //     z.vals[j] = y.vals_mut()[j + D].take();
    // }

    // z.size = D - 1;

    // // 子を付け替える
    // if let Some((y_children, z_children)) = y.children_mut().as_mut().zip(z.children.as_mut()) {
    //     for j in 0..D {
    //         z_children[j] = y_children[j + D].take();
    //     }
    // }

    // *y.size_mut() = D - 1;

    // // xのi番目より右の子を1つづつ右にずらす
    // for j in (i + 1..x.size + 1).rev() {
    //     x_children[j + 1] = x_children[j].take();
    // }

    // // zをxのi+1番目の子にする
    // x_children[i + 1] = Some(Rc::new(RefCell::new(z)));

    // // xのi番目より右のキー，値を1つづつ右にずらす
    // for j in (i..x.size).rev() {
    //     x.keys[j + 1] = x.keys[j].take();
    //     x.vals[j + 1] = x.vals[j].take();
    // }

    // x.keys[i] = y.keys_mut()[D - 1].take();
    // x.vals[i] = y.vals_mut()[D - 1].take();

    // x.size += 1;
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        btree,
        debug::print_as_tree,
        node::{BTreeNode, NodePtr},
    };

    use super::remove_from_leaf;

    #[test]
    fn test_remove_from_leaf() {
        let tree: Option<NodePtr<4, char, i32>> = btree! {
            keys: [Some('a'), Some('c'), Some('e'), Some('g'), Some('i'), Some('k'), None],
            vals: [Some(434), Some(112), Some(605), Some(705), Some(334), Some(963), None],
            size: 6
        };

        print_as_tree(&tree);

        for c in ['g', 'x', 'i', 'a', 'c', 'e', 'a', 'k'] {
            println!("> remove \'{c}\'");
            let res = remove_from_leaf(&mut *tree.as_ref().unwrap().borrow_mut(), &c);

            print_as_tree(&tree);
            println!("removed (key, val) = {:?}", res);

            if let Some((k, _)) = res {
                assert_eq!(k, c);
            }
        }
    }
}
