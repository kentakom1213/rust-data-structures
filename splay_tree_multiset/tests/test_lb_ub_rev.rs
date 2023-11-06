#![cfg(test)]

use std::array;
use std::cmp::Reverse;

use rand::*;
use splay_tree_multiset::multiset::*;
use splay_tree_multiset::tree;
use superslice::Ext;

#[test]
fn test_binary_search_non_duplicate() {
    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = (0..5).collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound_rev 0 ---");
    let lb0 = multiset.lower_bound_rev(&0);
    assert_eq!(lb0, Some(&0));
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = (0..5).collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound_rev 0 ---");
    let ub0 = multiset.upper_bound_rev(&0);
    assert!(ub0.is_none());
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = (0..5).collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound_rev 3 ---");
    let lb3 = multiset.lower_bound_rev(&3);
    assert_eq!(lb3, Some(&3));
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = (0..5).collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound_rev 3 ---");
    let ub3 = multiset.upper_bound_rev(&3);
    assert_eq!(ub3, Some(&2));
    println!("{:?}", &multiset);
}

#[test]
fn test_binary_search_duplicate() {
    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [3, 3, 4, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound (rev) 3 ---");
    let lb3 = multiset.lower_bound_rev(&3);
    assert_eq!(lb3, Some(&3));
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [3, 3, 4, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound (rev) 3 ---");
    let ub3 = multiset.upper_bound_rev(&3);
    assert_eq!(ub3, None);
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [3, 3, 4, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound (rev) 4 ---");
    let lb4 = multiset.lower_bound_rev(&4);
    assert_eq!(lb4, Some(&4));
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: SplayTreeMultiSet<usize> = [3, 3, 4, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound (rev) 4 ---");
    let ub4 = multiset.upper_bound_rev(&4);
    assert_eq!(ub4, Some(&3));
    println!("{:?}", &multiset);
}

#[test]
fn test_lb_ub_random() {
    const ARRAY_SIZE: usize = 2_000;
    const TEST_SIZE: usize = 100_000;

    let array: Vec<u8> = (0..ARRAY_SIZE).map(|_| random()).collect();

    let mut multiset: SplayTreeMultiSet<u8> = array.iter().cloned().collect();

    // Reverseをかける
    let mut array: Vec<Reverse<u8>> = array.into_iter().map(|v| Reverse(v)).collect();

    // 配列をソート
    array.sort();

    for _ in 0..TEST_SIZE {
        let x: u8 = random();

        // lower_bound
        let lb_expected = array.lower_bound(&Reverse(x));
        let lb_actual = multiset.lower_bound_rev(&x);

        assert_eq!(array.get(lb_expected).map(|v| &v.0), lb_actual);

        // upper_bound
        let ub_expected = array.upper_bound(&Reverse(x));
        let ub_actual = multiset.upper_bound_rev(&x);

        assert_eq!(array.get(ub_expected).map(|v| &v.0), ub_actual);
    }
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
    multiset.lower_bound_rev(&3);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 3 ---");
    multiset.upper_bound_rev(&3);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- lower_bound 4 ---");
    multiset.lower_bound_rev(&4);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 4 ---");
    multiset.upper_bound_rev(&4);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- lower_bound 5 ---");
    multiset.lower_bound(&5);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = SplayTreeMultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 5 ---");
    multiset.upper_bound_rev(&5);
    println!("{:#?}", multiset);
}
