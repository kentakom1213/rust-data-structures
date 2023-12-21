#![allow(non_snake_case)]

use std::{cmp::Ordering, fmt::Debug, mem::replace};

/// AA木のノード
#[derive(Debug)]
pub struct AATreeNodeInner<K, V>
where
    K: Ord,
{
    pub key: K,
    pub value: V,
    pub level: usize,
    pub left: Option<Box<AATreeNodeInner<K, V>>>,
    pub right: Option<Box<AATreeNodeInner<K, V>>>,
}

impl<K: Ord, V> AATreeNodeInner<K, V> {
    pub fn new(key: K, value: V) -> AATreeNode<K, V> {
        Some(Box::new(AATreeNodeInner {
            key,
            value,
            level: 1,
            left: None,
            right: None,
        }))
    }
}

pub type AATreeNode<K, V> = Option<Box<AATreeNodeInner<K, V>>>;

/// ノードの逆転
/// ```text
///   |        ⇓           ⇓        
/// 2 |    L ← T           L → T    
///   |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
/// 1 |  A   B   R       A   B   R  
/// ```
fn skew<K: Ord, V>(node: AATreeNode<K, V>) -> AATreeNode<K, V> {
    let Some(mut T) = node else {
        return None;
    };
    if T.left.is_none() {
        Some(T)
    } else if T.level == T.left.as_ref().unwrap().level {
        // ポインタの入れ替え
        let mut L = T.left.unwrap();
        T.left = L.right;
        L.right = Some(T);
        Some(L)
    } else {
        Some(T)
    }
}

/// ノードの分割操作
/// ```text
///   |                         ⇓    
/// 3 |                         R    
///   |    ⇓                   ↙ ↘   
/// 2 |    T → R → X   ==>    T   X  
///   |   ↙   ↙              ↙ ↘     
/// 1 |  A   B              A   B    
/// ```
fn split<K: Ord, V>(node: AATreeNode<K, V>) -> AATreeNode<K, V> {
    let Some(mut T) = node else {
        return None;
    };
    if T.right.is_none() || T.right.as_ref().unwrap().right.is_none() {
        Some(T)
    } else if T.level == T.right.as_ref().unwrap().right.as_ref().unwrap().level {
        let mut R = T.right.unwrap();
        T.right = R.left;
        R.left = Some(T);
        R.level += 1; // Rのレベルを1上げる
        Some(R)
    } else {
        Some(T)
    }
}

/// 値`key`に`value`を挿入する
/// - `root`: 挿入する木の根
fn insert<K: Ord, V>(root: AATreeNode<K, V>, key: K, value: V) -> AATreeNode<K, V> {
    let Some(mut T) = root else {
        return AATreeNodeInner::new(key, value);
    };
    match key.cmp(&T.key) {
        Ordering::Less => {
            T.left = insert(T.left, key, value);
        }
        Ordering::Greater => {
            T.right = insert(T.right, key, value);
        }
        Ordering::Equal => (),
    }
    let mut root = Some(T);
    root = skew(root);
    root = split(root);
    root
}

/// 値`key`を削除し，削除されたノードの`value`を返す
/// - `root`: 削除する木の根
fn delete<K: Ord, V>(root: AATreeNode<K, V>, key: &K) -> (AATreeNode<K, V>, Option<(K, V)>) {
    let Some(mut T) = root else {
        return (None, None);
    };
    let (new_root, old_key_value) = match key.cmp(&T.key) {
        Ordering::Less => {
            let (new_left, old_key_value) = delete(T.left, key);
            T.left = new_left;
            (Some(T), old_key_value)
        }
        Ordering::Greater => {
            let (new_right, old_key_value) = delete(T.right, key);
            T.right = new_right;
            (Some(T), old_key_value)
        }
        Ordering::Equal => {
            if T.left.is_none() {
                (T.right, Some((T.key, T.value)))
            } else if T.right.is_none() {
                (T.left, Some((T.key, T.value)))
            } else {
                // 左右の子を持つ場合，左の子の最大値を現在のノードに代入
                let (new_root, right_most) = delete_and_get_max(T.left);
                let AATreeNodeInner {
                    key: new_key,
                    value: new_value,
                    ..
                } = right_most.unwrap();
                let mut T = new_root.unwrap();
                let old_key = replace(&mut T.key, new_key);
                let old_value = replace(&mut T.value, new_value);
                (Some(T), Some((old_key, old_value)))
            }
        }
    };
    // バランスの修正
    let Some(mut T) = new_root else {
        return (None, old_key_value);
    };
    let left_level = T.left.as_ref().map_or(0, |node| node.level);
    let right_level = T.right.as_ref().map_or(0, |node| node.level);
    if left_level < T.level - 1 || right_level < T.level - 1 {
        T.level -= 1;
        // 右が大きい場合，下げる
        if right_level > T.level {
            T.right.as_mut().unwrap().level = T.level;
        }
        // 右のノードをskew
        T = skew(Some(T)).unwrap();
        T.right = skew(T.right);
        if let Some(mut right) = T.right.take() {
            right.right = skew(right.right);
            T.right.replace(right);
        }
        // 右のノードをsplit
        T = split(Some(T)).unwrap();
        T.right = split(T.right);
    }
    (Some(T), old_key_value)
}

/// nodeを根とする木のうち，値が最大のものを削除する
/// - 戻り値: (新しい根, 削除されたノード)
fn delete_and_get_max<K: Ord, V>(
    root: AATreeNode<K, V>,
) -> (AATreeNode<K, V>, Option<AATreeNodeInner<K, V>>) {
    let Some(mut T) = root else {
        return (None, None);
    };
    if T.right.is_none() {
        return (None, Some(*T));
    }
    let right_most = {
        let mut par = T.right.as_deref_mut().unwrap();
        while par.right.as_ref().unwrap().right.is_some() {
            par = par.right.as_deref_mut().unwrap();
        }
        *par.right.take().unwrap()
    };
    (Some(T), Some(right_most))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{print_util::pretty_print, tree};

    #[test]
    fn test_skew() {
        let mut tree = tree! {
            key: "D",
            value: 4,
            level: 2,
            left: tree! {
                key: "B",
                value: 2,
                level: 2,
                left: tree! {
                    key: "A",
                    value: 1,
                    level: 1,
                },
                right: tree! {
                    key: "C",
                    value: 3,
                    level: 1,
                },
            },
            right: tree! {
                key: "E",
                value: 5,
                level: 1,
            }
        };

        println!(" before skew ");
        println!("{:#?}", &tree);

        // skew
        tree = skew(tree);

        println!(" after skew ");
        println!("{:#?}", &tree);
    }

    #[test]
    fn test_split() {
        let mut tree = tree! {
            key: "B",
            value: 2,
            level: 2,
            left: tree! {
                key: "A",
                value: 1,
                level: 1,
            },
            right: tree! {
                key: "D",
                value: 4,
                level: 2,
                left: tree! {
                    key: "C",
                    value: 3,
                    level: 1,
                },
                right: tree! {
                    key: "E",
                    value: 5,
                    level: 2,
                }
            }
        };

        println!(" before split ");
        println!("{:#?}", &tree);

        // split
        tree = split(tree);

        println!(" after split ");
        println!("{:#?}", &tree);
    }

    #[test]
    fn test_insert() {
        let mut tree = None;

        println!("  default ");
        pretty_print(&tree);

        for (i, c) in ('A'..='Z').enumerate() {
            tree = insert(tree, c, i);

            println!("> insert {c}");
            pretty_print(&tree);
        }
    }

    #[test]
    fn test_insert_rev() {
        let mut tree = None;

        println!("  default");
        pretty_print(&tree);

        for (i, c) in ('A'..='Z').rev().enumerate() {
            tree = insert(tree, c, i);

            println!("> insert {c}");
            pretty_print(&tree);
        }
    }

    #[test]
    fn test_delete() {
        let mut tree = None;

        for (i, c) in ('A'..='H').enumerate() {
            tree = insert(tree, c, i);
        }

        println!("  default ");
        pretty_print(&tree);

        for c in 'A'..='J' {
            println!("> delete {c}");

            tree = delete(tree, &c).0;

            pretty_print(&tree);
        }
    }
}
