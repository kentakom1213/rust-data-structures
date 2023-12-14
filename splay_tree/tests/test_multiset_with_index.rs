use rand::random;
use splay_tree::multiset_with_index::*;

#[test]
fn test_count_hand() {
    let mut multiset = MultiSet::<u8>::new();

    eprintln!("{:?}", multiset);
    multiset.insert(0);
    eprintln!("{:?}", multiset);
    multiset.insert(0);
    eprintln!("{:?}", multiset);
    multiset.insert(0);
    eprintln!("{:?}", multiset);
    multiset.insert(20);
    eprintln!("{:?}", multiset);
    multiset.insert(20);
    eprintln!("{:?}", multiset);
    multiset.insert(20);
    eprintln!("{:?}", multiset);
    multiset.insert(20);
    eprintln!("{:?}", multiset);
    multiset.delete(&20);
    eprintln!("{:?}", multiset);
    multiset.delete(&10);
    eprintln!("{:?}", multiset);
    multiset.delete(&0);
    eprintln!("{:?}", multiset);
    multiset.delete(&0);
    eprintln!("{:?}", multiset);
}

#[test]
fn test_random_insert_delete_size() {
    const QUERY_SIZE: usize = 30;

    let mut multiset = MultiSet::<u8>::new();

    for _ in 0..QUERY_SIZE {
        let x = random();

        // multisetに挿入
        multiset.insert(x);

        eprintln!("----- size: {} -----", multiset.len());
        eprintln!("{:?}", multiset);

        let y = random();

        // multisetに挿入
        multiset.delete(&y);

        eprintln!("----- size: {} -----", multiset.len());
        eprintln!("{:?}", multiset);
    }
}
