#![cfg(test)]

use rand::*;
use splay_tree_simple::multiset::*;
use splay_tree_simple::tree_multiset;
use superslice::Ext;

#[test]
fn test_binary_search_non_duplicate() {
    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = (0..5).collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound_rev 0 ---");
    let lb0 = multiset.lower_bound_rev(&0);
    assert_eq!(lb0, Some(&0));
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = (0..5).collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound_rev 0 ---");
    let ub0 = multiset.upper_bound_rev(&0);
    assert!(ub0.is_none());
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = (0..5).collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound_rev 3 ---");
    let lb3 = multiset.lower_bound_rev(&3);
    assert_eq!(lb3, Some(&3));
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = (0..5).collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound_rev 3 ---");
    let ub3 = multiset.upper_bound_rev(&3);
    assert_eq!(ub3, Some(&2));
    println!("{:?}", &multiset);
}

#[test]
fn test_binary_search_duplicate() {
    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [3, 3, 4, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound (rev) 3 ---");
    let lb3 = multiset.lower_bound_rev(&3);
    assert_eq!(lb3, Some(&3));
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [3, 3, 4, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound (rev) 3 ---");
    let ub3 = multiset.upper_bound_rev(&3);
    assert_eq!(ub3, None);
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [3, 3, 4, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- lower_bound (rev) 4 ---");
    let lb4 = multiset.lower_bound_rev(&4);
    assert_eq!(lb4, Some(&4));
    println!("{:?}", &multiset);

    println!("### Reset ###");
    let mut multiset: MultiSet<usize> = [3, 3, 4, 4, 4, 5].into_iter().collect();
    println!("{:?}", &multiset);

    println!("--- upper_bound (rev) 4 ---");
    let ub4 = multiset.upper_bound_rev(&4);
    assert_eq!(ub4, Some(&3));
    println!("{:?}", &multiset);
}

#[test]
fn test_lb_ub_random() {
    const ARRAY_SIZE: usize = 10_000;
    const TEST_SIZE: usize = 10_000;

    let mut array: Vec<u8> = (0..ARRAY_SIZE).map(|_| random()).collect();

    let mut multiset: MultiSet<u8> = array.iter().cloned().collect();

    // 配列をソート
    array.sort();

    for _ in 0..TEST_SIZE {
        let x: u8 = random();

        // lower_bound
        let lb_expected_idx = array.upper_bound(&x) - 1;
        let lb_expected = array.get(lb_expected_idx).map(|v| *v);
        let lb_actual = multiset.lower_bound_rev(&x).map(|v| *v);

        assert_eq!(lb_expected, lb_actual);

        // upper_bound
        if let Some(ub_expected_idx) = array.lower_bound(&x).checked_sub(1) {
            let ub_expected = array.get(ub_expected_idx).map(|v| *v);
            let ub_actual = multiset.upper_bound_rev(&x).map(|v| *v);

            assert_eq!(ub_expected, ub_actual);
        }
    }
}

#[test]
fn test_binary_search_complete_splay() {
    let complete = tree_multiset! {
        key: 4,
        left: tree_multiset! {
            key: 2,
            left: tree_multiset! {
                key: 1,
            },
            right: tree_multiset! {
                key: 3,
            }
        },
        right: tree_multiset! {
            key: 6,
            left: tree_multiset! {
                key: 5,
            },
            right: tree_multiset! {
                key: 7,
            }
        }
    };

    println!("### Reset ###");
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- lower_bound 3 ---");
    multiset.lower_bound_rev(&3);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 3 ---");
    multiset.upper_bound_rev(&3);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- lower_bound 4 ---");
    multiset.lower_bound_rev(&4);
    println!("{:#?}", multiset);

    println!("### Reset ###");
    let mut multiset = MultiSet::<usize>::new();
    multiset.root = complete.clone();
    println!("{:#?}", &multiset);

    println!("--- upper_bound 4 ---");
    multiset.upper_bound_rev(&4);
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
    multiset.upper_bound_rev(&5);
    println!("{:#?}", multiset);
}
