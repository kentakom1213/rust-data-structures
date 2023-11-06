use rand::random;
use splay_tree_multiset::multiset::*;
use superslice::Ext;

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
fn test_insert_sorted_greater() {
    let multiset: SplayTreeMultiSet<usize> = (0..20).collect();

    println!("{:#?}", multiset);
}

#[test]
fn test_insert_sorted_less() {
    let multiset: SplayTreeMultiSet<usize> = (0..20).rev().collect();

    println!("{:#?}", multiset);
}

#[test]
fn test_random_insert() {
    const INSERT_SIZE: usize = 10_000;

    let mut array: Vec<u8> = vec![];
    let mut multiset = SplayTreeMultiSet::<u8>::new();

    for _ in 0..INSERT_SIZE {
        let x = random();

        // arrayにinsert
        let idx = array.lower_bound(&x);
        array.insert(idx, x);

        // multisetにinsert
        multiset.insert(x);

        assert_eq!(array.len(), multiset.len());
    }
}

#[test]
fn test_random_delete() {
    const INSERT_SIZE: usize = 10_000;
    const QUERY_SIZE: usize = 10_000;

    let mut array = vec![];
    let mut multiset = SplayTreeMultiSet::<u8>::new();

    // random insert
    for _ in 0..INSERT_SIZE {
        let x = random();
        array.push(x);
        multiset.insert(x);
    }

    // arrayのソート
    array.sort();

    assert_eq!(array.len(), multiset.len());

    // random insert and delete
    for _ in 0..QUERY_SIZE {
        let x = random();

        // arrayから削除
        if let Ok(idx) = array.binary_search(&x) {
            array.remove(idx);
        }

        // multisetから削除
        multiset.delete(&x);

        assert_eq!(array.len(), multiset.len());
    }
}
