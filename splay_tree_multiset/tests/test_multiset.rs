#![cfg(test)]

use splay_tree_multiset::multiset::*;

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
fn test_insert_delete_non_duplicate() {
    let mut multiset: SplayTreeMultiSet<usize> = (0..5).collect();

    println!("{:?}", &multiset);

    println!("--- lower_bound 3 ---");
    let lb3 = multiset.lower_bound(&3);
    println!("{:?}", lb3);

    println!("--- upper_bound 3 ---");
    let ub3 = multiset.upper_bound(&3);
    println!("{:?}", ub3);

    println!("--- lower_bound 4 ---");
    let lb4 = multiset.lower_bound(&4);
    println!("{:?}", lb4);

    println!("--- upper_bound 4 ---");
    let ub4 = multiset.upper_bound(&4);
    println!("{:?}", ub4);
}

#[test]
fn test_insert_delete_duplicate() {
    let mut multiset: SplayTreeMultiSet<usize> = [0, 1, 3, 3, 4, 4].into_iter().collect();

    println!("{:?}", &multiset);

    println!("--- lower_bound 3 ---");
    let lb3 = multiset.lower_bound(&3);
    println!("{:?}", lb3);

    println!("--- upper_bound 3 ---");
    let ub3 = multiset.upper_bound(&3);
    println!("{:?}", ub3);

    println!("--- lower_bound 4 ---");
    let lb4 = multiset.lower_bound(&4);
    println!("{:?}", lb4);

    println!("--- upper_bound 4 ---");
    let ub4 = multiset.upper_bound(&4);
    println!("{:?}", ub4);
}
