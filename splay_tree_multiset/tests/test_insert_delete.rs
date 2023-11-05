use rand::random;
use splay_tree_multiset::multiset::{*, self};
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
fn test_insert_random() {
    let mut multiset = SplayTreeMultiSet::<u8>::new();

    for _ in 0..50 {
        let x = random();
        multiset.insert(x);

        println!("--- insert {} ---", x);
        println!("{:#?}", multiset);
    }
}

#[test]
fn test_random_delete() {
    const INSERT_SIZE: usize = 10;
    const QUERY_SIZE: usize = 5;

    let mut array = vec![];
    let mut multiset = SplayTreeMultiSet::<u8>::new();

    // random insert
    for _ in 0..INSERT_SIZE {
        let x = random();
        array.push(x);
        multiset.insert(x);
    }

    println!("{:?}", array);

    // arrayのソート
    array.sort();

    assert_eq!(array.len(), multiset.len());

    // random insert and delete
    for _ in 0..QUERY_SIZE {
        let x = random();

        println!("--- del {} ---", x);

        // arrayから削除
        if let Ok(idx) = array.binary_search(&x) {
            array.remove(idx);
        }

        // multisetから削除
        multiset.delete(&x);

        println!("{:?}", array);
        println!("{:?}", multiset);

        assert_eq!(array.len(), multiset.len());
    }
}
