#![cfg(test)]

use splay_tree_multiset::multiset::*;
use splay_tree_multiset::tree;

#[test]
fn test_insert() {
    let mut multiset = SplayTreeMultiSet::new();

    assert_eq!(multiset.len(), 0);

    println!("--- insert 1 ---");
    multiset.insert(1);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 1);

    println!("--- insert 1 ---");
    multiset.insert(1);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 2);

    println!("--- insert 1 ---");
    multiset.insert(1);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 3);

    println!("--- insert 1 ---");
    multiset.insert(1);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 4);

    println!("--- insert 0 ---");
    multiset.insert(0);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 5);

    println!("--- insert 2 ---");
    multiset.insert(2);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 6);
}

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
