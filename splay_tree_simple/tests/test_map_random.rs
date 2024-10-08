#![cfg(test)]

use rand::prelude::*;
use splay_tree_simple::map::*;
use std::collections::BTreeMap;

#[test]
fn test_random_insert_get() {
    const CASE: usize = 100_000;

    let mut std_map: BTreeMap<u16, u16> = BTreeMap::new();
    let mut splay_map: SplayTreeMap<u16, u16> = SplayTreeMap::new();

    for _ in 0..CASE {
        // 挿入
        let x = random();

        std_map.insert(x, x >> 3);
        splay_map.insert(x, x >> 3);

        // 検索
        let y = random();

        assert_eq!(std_map.get(&y), splay_map.get(&y));
    }
}

#[test]
fn test_random_insert_delete() {
    const CASE: usize = 100_000;

    let mut std_map: BTreeMap<u16, u16> = BTreeMap::new();
    let mut splay_map: SplayTreeMap<u16, u16> = SplayTreeMap::new();

    for _ in 0..CASE {
        // 挿入
        let x = random();

        std_map.insert(x, x >> 3);
        splay_map.insert(x, x >> 3);

        // 要素数の確認
        assert_eq!(std_map.len(), splay_map.len());

        // 削除
        let y = random();

        assert_eq!(std_map.remove(&y), splay_map.delete(&y));

        // 要素数の確認
        assert_eq!(std_map.len(), splay_map.len());
    }
}
