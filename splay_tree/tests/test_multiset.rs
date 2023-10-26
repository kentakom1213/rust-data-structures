#![cfg(test)]

use splay_tree::splay_tree_multiset::*;

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
fn test_insert_delete() {
    let pi = "314159265358979323846264338327950288419";

    let mut multiset: SplayTreeMultiSet<char> = pi.chars().collect();

    println!("{:?}", &multiset);

    assert_eq!(pi.len(), multiset.len());

    println!("--- lower_bound 9 ---");
    multiset.lower_bound(&'9');
    println!("{:?}", &multiset);

    println!("--- upper_bound 9 ---");
    multiset.upper_bound(&'9');
    println!("{:?}", &multiset);

    println!("--- lower_bound 9 ---");
    multiset.lower_bound(&'9');
    println!("{:?}", &multiset);

}
