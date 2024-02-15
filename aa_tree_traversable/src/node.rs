#![allow(non_snake_case)]

use std::{
    cell::RefCell,
    cmp::Ordering,
    rc::{Rc, Weak},
};

pub type AATreeNode<K, V> = Option<Rc<RefCell<AATreeNodeInner<K, V>>>>;
pub type AATreeNodeParent<K, V> = Option<Weak<RefCell<AATreeNodeInner<K, V>>>>;

/// AA木のノード
#[derive(Debug)]
pub struct AATreeNodeInner<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub level: usize,
    pub parent: AATreeNodeParent<K, V>,
    pub left: AATreeNode<K, V>,
    pub right: AATreeNode<K, V>,
}

impl<K: Ord, V> AATreeNodeInner<K, V> {
    pub fn new(key: K, value: V) -> AATreeNode<K, V> {
        Some(Rc::new(RefCell::new(AATreeNodeInner {
            key,
            value,
            level: 1,
            parent: None,
            left: None,
            right: None,
        })))
    }
}

/// skew操作
///
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
        // Bがあるときの処理
        if let Some(B) = L.borrow_mut().right.take() {
            // Bの親をTに
            B.borrow_mut().parent.replace(Rc::downgrade(&T));
            // Tの左の子をBに
            T.borrow_mut().left.replace(B);
        }
        // Tの親をLに
        T.borrow_mut().parent.replace(Rc::downgrade(&L));
        // Lの右の子をTに
        L.borrow_mut().right.replace(T);
        Some(L)
    } else {
        Some(T)
    }
}

/// split操作
///
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
        // Bがあるときの処理
        if let Some(B) = R.borrow_mut().left.take() {
            // Bの親をTに
            B.borrow_mut().parent.replace(Rc::downgrade(&T));
            // Tの右の子をBに
            T.borrow_mut().right.replace(B);
        }
        // Tの親をRに
        T.borrow_mut().parent.replace(Rc::downgrade(&R));
        // Rの左の子をTに
        R.borrow_mut().left.replace(T);
        // Rのレベルを1上げる
        R.borrow_mut().level += 1;
        Some(R)
    } else {
        Some(T)
    }
}

/// 値`key`に`value`を挿入する
/// - `root`: 挿入する木の根
///
/// ```text
///    ⇓    
///    T    
///   ↙ ↘   
///  A   B  
/// ```
pub fn insert<K: Ord, V>(root: AATreeNode<K, V>, key: K, value: V) -> AATreeNode<K, V> {
    let Some(T) = root else {
        return AATreeNodeInner::new(key, value);
    };
    let order = key.cmp(&T.borrow().key);
    match order {
        Ordering::Less => {
            let left = T.borrow_mut().left.take();
            if let Some(A) = insert(left, key, value) {
                if A.borrow().parent.is_none() {
                    A.borrow_mut().parent.replace(Rc::downgrade(&T));
                }
                T.borrow_mut().left.replace(A);
            }
        }
        Ordering::Greater => {
            let right = T.borrow_mut().right.take();
            if let Some(B) = insert(right, key, value) {
                if B.borrow().parent.is_none() {
                    B.borrow_mut().parent.replace(Rc::downgrade(&T));
                }
                T.borrow_mut().right.replace(B);
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
mod test_aatree_traverse {
    use super::*;
    use crate::{print_util::print_as_binary_tree, tree};

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
        // println!("{:#?}", &tree);
        print_as_binary_tree(&tree);

        // skew
        tree = skew(tree);

        println!("--- after skew ---");
        // println!("{:#?}", &tree);
        print_as_binary_tree(&tree);
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
        // println!("{:#?}", &tree);
        print_as_binary_tree(&tree);

        // split
        tree = split(tree);

        println!("--- after split ---");
        // println!("{:#?}", &tree);
        print_as_binary_tree(&tree);
    }

    #[test]
    fn test_insert_and_traverse() {
        let mut tree = None;

        for (v, k) in ('a'..='e').enumerate() {
            tree = insert(tree, k, v);
        }

        print_as_binary_tree(&tree);

        // traverse
        let mut iter = &tree;

        let key = iter.as_ref().unwrap().borrow().key;
        let value = iter.as_ref().unwrap().borrow().value;
        println!("key: {}, value: {}", key, value);

        // next
        let bind = &iter.as_ref().unwrap().borrow();
        let tmp = &bind.left;

        println!("{:?}", iter);

        // // traverse
        // let key = iter.as_ref().unwrap().borrow().key;
        // let value = iter.as_ref().unwrap().borrow().value;
        // println!("key: {}, value: {}", key, value);

        // // next
        // iter = &iter.as_ref().unwrap().clone().borrow().left;

        // // traverse
        // let key = iter.as_ref().unwrap().borrow().key;
        // let value = iter.as_ref().unwrap().borrow().value;
        // println!("key: {}, value: {}", key, value);
    }
}
