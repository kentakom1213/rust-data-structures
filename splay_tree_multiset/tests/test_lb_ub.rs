#![cfg(test)]

use itertools::Itertools;
use rand::*;
use splay_tree_multiset::multiset;
use splay_tree_multiset::multiset::*;
use splay_tree_multiset::tree;
use superslice::Ext;

#[test]
fn test_binary_search_non_duplicate() {
    let mut multiset: SplayTreeMultiSet<usize> = (0..5).collect();

    println!("{:?}", &multiset);

    println!("--- lower_bound 3 ---");
    let lb3 = multiset.lower_bound(&3);
    println!("{:?}", lb3);
    assert_eq!(lb3.as_ref().unwrap().key, 3);

    println!("--- upper_bound 3 ---");
    let ub3 = multiset.upper_bound(&3);
    println!("{:?}", ub3);
    assert_eq!(ub3.as_ref().unwrap().key, 4);

    println!("--- lower_bound 4 ---");
    let lb4 = multiset.lower_bound(&4);
    println!("{:?}", lb4);
    assert_eq!(lb4.as_ref().unwrap().key, 4);

    println!("--- upper_bound 4 ---");
    let ub4 = multiset.upper_bound(&4);
    println!("{:?}", ub4);
    assert!(ub4.is_none());
}

#[test]
fn test_binary_search_duplicate() {
    let mut multiset: SplayTreeMultiSet<usize> = [0, 1, 3, 3, 4, 4].into_iter().collect();

    println!("{:?}", &multiset);

    println!("--- lower_bound 3 ---");
    let lb3 = multiset.lower_bound(&3);
    println!("{:?}", lb3);
    assert_eq!(lb3.as_ref().unwrap().key, 3);

    println!("--- upper_bound 3 ---");
    let ub3 = multiset.upper_bound(&3);
    println!("{:?}", ub3);
    assert_eq!(ub3.as_ref().unwrap().key, 4);

    println!("--- lower_bound 4 ---");
    let lb4 = multiset.lower_bound(&4);
    println!("{:?}", lb4);
    assert_eq!(lb4.as_ref().unwrap().key, 4);

    println!("--- upper_bound 4 ---");
    let ub4 = multiset.upper_bound(&4);
    println!("{:?}", ub4);
    assert!(ub4.is_none());
}

#[test]
fn test_binary_search_complete() {
    let mut multiset = SplayTreeMultiSet::new();

    multiset.root = tree! {
        key: 4,
        left: tree! {
            key: 2,
            left: tree! {
                key: 1,
            },
            right: tree! {
                key: 3,
            }
        },
        right: tree! {
            key: 6,
            left: tree! {
                key: 5,
            },
            right: tree! {
                key: 7,
            }
        }
    };

    println!("{:#?}", &multiset);

    println!("--- lower_bound 3 ---");
    let lb3 = multiset.lower_bound(&3);
    println!("{:?}", lb3);
    assert_eq!(lb3.as_ref().unwrap().key, 3);

    println!("--- upper_bound 3 ---");
    let ub3 = multiset.upper_bound(&3);
    println!("{:?}", ub3);
    assert_eq!(ub3.as_ref().unwrap().key, 4);

    println!("--- lower_bound 4 ---");
    let lb4 = multiset.lower_bound(&4);
    println!("{:?}", lb4);
    assert_eq!(lb4.as_ref().unwrap().key, 4);

    println!("--- upper_bound 4 ---");
    let ub4 = multiset.upper_bound(&4);
    println!("{:?}", ub4);
    assert_eq!(ub4.as_ref().unwrap().key, 5);

    println!("--- lower_bound 5 ---");
    let lb5 = multiset.lower_bound(&5);
    println!("{:?}", lb5);
    assert_eq!(lb5.as_ref().unwrap().key, 5);

    println!("--- upper_bound 5 ---");
    let ub5 = multiset.upper_bound(&5);
    println!("{:?}", ub5);
    assert_eq!(ub5.as_ref().unwrap().key, 6);
}

#[test]
fn test_lb_ub_random() {
    const ARRAY_SIZE: usize = 2_000;
    const TEST_SIZE: usize = 100_000;

    let mut array: Vec<u8> = (0..ARRAY_SIZE).map(|_| random()).collect();

    // println!("{:?}", array);

    let mut multiset: SplayTreeMultiSet<u8> = array.iter().cloned().collect();

    // 配列をソート
    array.sort();

    // println!("{:?}", array);
    // println!("{:?}", multiset);

    for _ in 0..TEST_SIZE {
        let x: u8 = random();

        // lower_bound
        let lb_expected = array.lower_bound(&x);
        let lb_actual = multiset.lower_bound(&x);

        if let Some(&expected_val) = array.get(lb_expected) {
            let actual_val = lb_actual.as_ref().unwrap().key;
            assert_eq!(expected_val, actual_val);
        } else {
            assert!(lb_actual.is_none());
        }
        
        // upper_bound
        let ub_expected = array.upper_bound(&x);
        let ub_actual = multiset.upper_bound(&x);

        if let Some(&expected_val) = array.get(ub_expected) {
            let actual_val = ub_actual.as_ref().unwrap().key;
            assert_eq!(expected_val, actual_val);
        } else {
            assert!(ub_actual.is_none());
        }  
    }
}

#[test]
fn test_binary_search_duplicate_splay() {
    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [0, 1, 3, 3, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound 3 ---");
    multiset.lower_bound_splay(&3);
    println!("{:?}", &multiset);
    // assert_eq!(lb3.as_ref().unwrap().key, 3);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [0, 1, 3, 3, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound 3 ---");
    multiset.upper_bound_splay(&3);
    println!("{:?}", &multiset);
    // assert_eq!(ub3.as_ref().unwrap().key, 4);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [0, 1, 3, 3, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound 4 ---");
    multiset.lower_bound_splay(&4);
    println!("{:?}", &multiset);
    // assert_eq!(lb4.as_ref().unwrap().key, 4);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [0, 1, 3, 3, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound 4 ---");
    multiset.upper_bound_splay(&4);
    println!("{:?}", &multiset);
    // assert!(ub4.is_none());
}


#[test]
fn test_binary_search_duplicate_right_splay() {
    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [5, 4, 4, 3, 3, 1, 0].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound 3 ---");
    multiset.lower_bound_splay(&3);
    println!("{:?}", &multiset);
    // assert_eq!(lb3.as_ref().unwrap().key, 3);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [5, 4, 4, 3, 3, 1, 0].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound 3 ---");
    multiset.upper_bound_splay(&3);
    println!("{:?}", &multiset);
    // assert_eq!(ub3.as_ref().unwrap().key, 4);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [5, 4, 4, 3, 3, 1, 0].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound 4 ---");
    multiset.lower_bound_splay(&4);
    println!("{:?}", &multiset);
    // assert_eq!(lb4.as_ref().unwrap().key, 4);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [5, 4, 4, 3, 3, 1, 0].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound 4 ---");
    multiset.upper_bound_splay(&4);
    println!("{:?}", &multiset);
    // assert!(ub4.is_none());
}

#[test]
fn test_binary_search_complete_splay() {
    let complete = tree! {
        key: 4,
        left: tree! {
            key: 2,
            left: tree! {
                key: 1,
            },
            right: tree! {
                key: 3,
            }
        },
        right: tree! {
            key: 6,
            left: tree! {
                key: 5,
            },
            right: tree! {
                key: 7,
            }
        }
    };

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- lower_bound 3 ---");
    multiset.lower_bound_splay(&3);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 3 ---");
    multiset.upper_bound_splay(&3);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- lower_bound 4 ---");
    multiset.lower_bound_splay(&4);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 4 ---");
    multiset.upper_bound_splay(&4);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);
    
    println!("--- lower_bound 5 ---");
    multiset.lower_bound_splay(&5);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);
    
    println!("--- upper_bound 5 ---");
    multiset.upper_bound_splay(&5);
    println!("{:#?}", multiset);
}

