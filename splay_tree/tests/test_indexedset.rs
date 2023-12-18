#![cfg(test)]

use rand::*;
use splay_tree::indexedset::*;
use std::collections::BTreeSet;

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

    assert_eq!(set.nth(0), Some(&(-5)));
    assert_eq!(set.nth(1), Some(&(1)));
    assert_eq!(set.nth(2), Some(&(4)));
    assert_eq!(set.nth(3), Some(&(5)));
    assert_eq!(set.nth(4), Some(&(10)));
    assert_eq!(set.nth(5), Some(&(20)));
    assert_eq!(set.nth(6), Some(&(32)));
    assert_eq!(set.nth(7), Some(&(100)));
    assert_eq!(set.nth(8), Some(&(256)));
    assert_eq!(set.nth(9), Some(&(1024)));

    set.insert(512);

    eprintln!("{:?}", set);

    assert_eq!(set.nth(0), Some(&(-5)));
    assert_eq!(set.nth(1), Some(&(1)));
    assert_eq!(set.nth(2), Some(&(4)));
    assert_eq!(set.nth(3), Some(&(5)));
    assert_eq!(set.nth(4), Some(&(10)));
    assert_eq!(set.nth(5), Some(&(20)));
    assert_eq!(set.nth(6), Some(&(32)));
    assert_eq!(set.nth(7), Some(&(100)));
    assert_eq!(set.nth(8), Some(&(256)));
    assert_eq!(set.nth(9), Some(&(512)));
    assert_eq!(set.nth(10), Some(&(1024)));
}
