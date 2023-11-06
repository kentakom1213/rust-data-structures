#![cfg(test)]

use rand::*;
use splay_tree_multiset::multiset::*;
use splay_tree_multiset::tree;
use superslice::Ext;

#[test]
fn test_lb_ub_random() {
    const ARRAY_SIZE: usize = 2_000;
    const TEST_SIZE: usize = 100_000;

    let mut array: Vec<u8> = (0..ARRAY_SIZE).map(|_| random()).collect();

    // println!("{:?}", array);

    let mut multiset: MultiSet<u8> = array.iter().cloned().collect();

    // 配列をソート
    array.sort();

    // println!("{:?}", array);
    // println!("{:?}", multiset);

    for _ in 0..TEST_SIZE {
        let x: u8 = random();

        // lower_bound
        let lb_expected = array.lower_bound(&x);
        let lb_actual = multiset.lower_bound(&x);

        assert_eq!(array.get(lb_expected), lb_actual);

        // upper_bound
        let ub_expected = array.upper_bound(&x);
        let ub_actual = multiset.upper_bound(&x);

        assert_eq!(array.get(ub_expected), ub_actual);
    }
}

#[test]
fn test_binary_search_duplicate_splay() {
    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [0, 1, 3, 3, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound 3 ---");
    multiset.lower_bound(&3);
    println!("{:?}", &multiset);
    // assert_eq!(lb3.as_ref().unwrap().key, 3);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [0, 1, 3, 3, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound 3 ---");
    multiset.upper_bound(&3);
    println!("{:?}", &multiset);
    // assert_eq!(ub3.as_ref().unwrap().key, 4);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [0, 1, 3, 3, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound 4 ---");
    multiset.lower_bound(&4);
    println!("{:?}", &multiset);
    // assert_eq!(lb4.as_ref().unwrap().key, 4);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [0, 1, 3, 3, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound 4 ---");
    multiset.upper_bound(&4);
    println!("{:?}", &multiset);
    // assert!(ub4.is_none());
}

#[test]
fn test_binary_search_duplicate_right_splay() {
    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [5, 4, 4, 3, 3, 1, 0].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound 3 ---");
    multiset.lower_bound(&3);
    println!("{:?}", &multiset);
    // assert_eq!(lb3.as_ref().unwrap().key, 3);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [5, 4, 4, 3, 3, 1, 0].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound 3 ---");
    multiset.upper_bound(&3);
    println!("{:?}", &multiset);
    // assert_eq!(ub3.as_ref().unwrap().key, 4);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [5, 4, 4, 3, 3, 1, 0].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound 4 ---");
    multiset.lower_bound(&4);
    println!("{:?}", &multiset);
    // assert_eq!(lb4.as_ref().unwrap().key, 4);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [5, 4, 4, 3, 3, 1, 0].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound 4 ---");
    multiset.upper_bound(&4);
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
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- lower_bound 3 ---");
    multiset.lower_bound(&3);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 3 ---");
    multiset.upper_bound(&3);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- lower_bound 4 ---");
    multiset.lower_bound(&4);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 4 ---");
    multiset.upper_bound(&4);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- lower_bound 5 ---");
    multiset.lower_bound(&5);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 5 ---");
    multiset.upper_bound(&5);
    println!("{:#?}", multiset);
}
