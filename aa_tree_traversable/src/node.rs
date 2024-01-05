#![allow(non_snake_case)]

use std::{
    cell::RefCell,
    cmp::Ordering,
    rc::{Rc, Weak},
};

pub type AATreeNode<K, V> = Option<Rc<RefCell<AATreeNodeInner<K, V>>>>;
pub type AATreeNodeParent<K, V> = Weak<RefCell<AATreeNodeInner<K, V>>>;

/// AA木のノード
#[derive(Debug)]
pub struct AATreeNodeInner<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub level: usize,
    // pub parent: AATreeNodeParent<K, V>,
    pub left: AATreeNode<K, V>,
    pub right: AATreeNode<K, V>,
}

impl<K: Ord, V> AATreeNodeInner<K, V> {
    pub fn new(key: K, value: V) -> AATreeNode<K, V> {
        Some(Rc::new(RefCell::new(AATreeNodeInner {
            key,
            value,
            level: 1,
            // parent: None,
            left: None,
            right: None,
        })))
    }
}

/// ノードの逆転
/// ```text
///   |        ⇓           ⇓        
/// 2 |    L ← T           L → T    
///   |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
/// 1 |  A   B   R       A   B   R  
/// ```
fn skew<K: Ord, V>(node: AATreeNode<K, V>) -> AATreeNode<K, V> {
    let Some(T) = node else {
        return node;
    };
    if T.borrow().left.is_none() {
        Some(T)
    } else if T.borrow().level == T.borrow().left.as_ref().unwrap().borrow().level {
        let L = T.borrow_mut().left.take().unwrap();
        if let Some(B) = L.borrow_mut().right.take() {
            T.borrow_mut().left.replace(B);
        }
        L.borrow_mut().right.replace(T);
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
    let Some(T) = node else {
        return None;
    };
    if T.borrow().right.is_none() || T.borrow().right.as_ref().unwrap().borrow().right.is_none() {
        Some(T)
    } else if T.borrow().level
        == T.borrow()
        .right
        .as_ref()
        .unwrap()
        .borrow()
        .right
        .as_ref()
        .unwrap()
        .borrow()
        .level
    {
        let R = T.borrow_mut().right.take().unwrap();
        if let Some(new_right) = R.borrow_mut().left.take() {
            T.borrow_mut().right.replace(new_right);
        }
        R.borrow_mut().left.replace(T);
        R.borrow_mut().level += 1; // Rのレベルを1上げる
        Some(R)
    } else {
        Some(T)
    }
}

/// 値`key`に`value`を挿入する
/// - `root`: 挿入する木の根
pub fn insert<K: Ord, V>(root: AATreeNode<K, V>, key: K, value: V) -> AATreeNode<K, V> {
    let Some(T) = root else {
        return AATreeNodeInner::new(key, value);
    };
    let order = key.cmp(&T.borrow().key);
    match order {
        Ordering::Less => {
            let left = T.borrow_mut().left.take();
            if let Some(new_left) = insert(left, key, value) {
                T.borrow_mut().left.replace(new_left);
            }
        }
        Ordering::Greater => {
            let right = T.borrow_mut().right.take();
            if let Some(new_right) = insert(right, key, value) {
                T.borrow_mut().right.replace(new_right);
            }
        }
        Ordering::Equal => {
            T.borrow_mut().value = value;
        }
    }
    let mut root = Some(T);
    root = skew(root);
    root = split(root);
    root
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

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

        println!("--- before skew ---");
        println!("{:#?}", &tree);

        // skew
        tree = skew(tree);

        println!("--- after skew ---");
        println!("{:#?}", &tree);
    }

    #[test]
    fn test_split() {
        let mut tree = tree! {
            key: "B",
            value: 1,
            level: 2,
            left: tree! {
                key: "A",
                value: 2,
                level: 1,
            },
            right: tree! {
                key: "D",
                value: 3,
                level: 2,
                left: tree! {
                    key: "C",
                    value: 4,
                    level: 1,
                },
                right: tree! {
                    key: "F",
                    value: 5,
                    level: 2,
                    left: tree! {
                        key: "E",
                        value: 6,
                        level: 1,
                    },
                    right: tree! {
                        key: "G",
                        value: 7,
                        level: 1,
                    }
                }
            }
        };

        println!("--- before split ---");
        println!("{:#?}", &tree);

        // split
        tree = split(tree);

        println!("--- after split ---");
        println!("{:#?}", &tree);
    }
}
