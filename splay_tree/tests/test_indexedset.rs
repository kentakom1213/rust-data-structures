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
