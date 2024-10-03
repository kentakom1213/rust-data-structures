#![cfg(test)]

use rand::*;
use splay_tree_simple::indexedset::*;
use std::collections::BTreeSet;
use superslice::Ext;

#[test]
fn test_random_insert_get() {
    const CASE: usize = 100_000;

    let mut std_set: BTreeSet<u8> = BTreeSet::new();
    let mut myset: IndexedSet<u8> = IndexedSet::new();

    for _ in 0..CASE {
        // 挿入
        let x = random();

        std_set.insert(x);
        myset.insert(x);

        // 検索
        let y = random();

        assert_eq!(std_set.get(&y), myset.get(&y));
    }
}

#[test]
fn test_random_insert_delete() {
    const CASE: usize = 100_000;

    let mut std_set: BTreeSet<u8> = BTreeSet::new();
    let mut myset: IndexedSet<u8> = IndexedSet::new();

    for _ in 0..CASE {
        // 挿入
        let x = random();

        std_set.insert(x);
        myset.insert(x);

        // 要素数の確認
        assert_eq!(std_set.len(), myset.len());

        // 削除
        let y = random();

        assert_eq!(std_set.remove(&y), myset.delete(&y).is_some());

        // 要素数の確認
        assert_eq!(std_set.len(), myset.len());
    }
}

#[test]
fn test_indexing() {
    let mut set = IndexedSet::from_iter([1, 4, 5, 10, 20, 100, 256, 1024, 10, -5, 32]);

    eprintln!("{:?}", set);

    assert_eq!(set.get_by_index(0), Some(&(-5)));
    assert_eq!(set.get_by_index(1), Some(&(1)));
    assert_eq!(set.get_by_index(2), Some(&(4)));
    assert_eq!(set.get_by_index(3), Some(&(5)));
    assert_eq!(set.get_by_index(4), Some(&(10)));
    assert_eq!(set.get_by_index(5), Some(&(20)));
    assert_eq!(set.get_by_index(6), Some(&(32)));
    assert_eq!(set.get_by_index(7), Some(&(100)));
    assert_eq!(set.get_by_index(8), Some(&(256)));
    assert_eq!(set.get_by_index(9), Some(&(1024)));

    set.insert(512);

    eprintln!("{:?}", set);

    assert_eq!(set.get_by_index(0), Some(&(-5)));
    assert_eq!(set.get_by_index(1), Some(&(1)));
    assert_eq!(set.get_by_index(2), Some(&(4)));
    assert_eq!(set.get_by_index(3), Some(&(5)));
    assert_eq!(set.get_by_index(4), Some(&(10)));
    assert_eq!(set.get_by_index(5), Some(&(20)));
    assert_eq!(set.get_by_index(6), Some(&(32)));
    assert_eq!(set.get_by_index(7), Some(&(100)));
    assert_eq!(set.get_by_index(8), Some(&(256)));
    assert_eq!(set.get_by_index(9), Some(&(512)));
    assert_eq!(set.get_by_index(10), Some(&(1024)));
}

#[test]
fn test_get_index() {
    const CASE: usize = 10_000;

    let mut arr = Vec::<u8>::new();
    let mut set = IndexedSet::<u8>::new();

    for _ in 0..CASE {
        let x = random();

        // 検索
        let arr_idx = arr.binary_search(&x).ok();
        let set_idx = set.index(&x);

        assert_eq!(arr_idx, set_idx);

        // 挿入
        if arr_idx.is_none() {
            let index = arr.lower_bound(&x);
            arr.insert(index, x);

            set.insert(x);
        }
    }
}

#[test]
fn test_iter() {
    let set = IndexedSet::from_iter([
        "gamma", "alpha", "epsilon", "phi", "theta", "omega", "beta", "delta", "pi", "tau", "mu",
        "eta", "kappa", "xi", "zeta",
    ]);

    let sorted = vec![
        "alpha", "beta", "delta", "epsilon", "eta", "gamma", "kappa", "mu", "omega", "phi", "pi",
        "tau", "theta", "xi", "zeta",
    ];

    assert_eq!(set.iter().cloned().collect::<Vec<&str>>(), sorted);
}
