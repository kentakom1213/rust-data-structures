use std::collections::BTreeMap;

use rand::random;
use splay_tree::multiset::*;

#[test]
fn test_count_hand() {
    let mut multiset = MultiSet::<u8>::new();

    assert_eq!(multiset.count(&0), 0);
    assert_eq!(multiset.count(&10), 0);
    assert_eq!(multiset.count(&20), 0);

    // 挿入
    multiset.insert(0);
    multiset.insert(0);
    multiset.insert(0);

    assert_eq!(multiset.count(&0), 3);
    assert_eq!(multiset.count(&10), 0);
    assert_eq!(multiset.count(&20), 0);

    // 挿入
    multiset.insert(20);
    multiset.insert(20);
    multiset.insert(20);
    multiset.insert(20);

    assert_eq!(multiset.count(&0), 3);
    assert_eq!(multiset.count(&10), 0);
    assert_eq!(multiset.count(&20), 4);

    // 削除
    multiset.delete(&20);

    assert_eq!(multiset.count(&0), 3);
    assert_eq!(multiset.count(&10), 0);
    assert_eq!(multiset.count(&20), 3);

    // 削除
    multiset.delete(&10);

    assert_eq!(multiset.count(&0), 3);
    assert_eq!(multiset.count(&10), 0);
    assert_eq!(multiset.count(&20), 3);

    // 削除
    multiset.delete(&0);
    multiset.delete(&0);

    assert_eq!(multiset.count(&0), 1);
    assert_eq!(multiset.count(&10), 0);
    assert_eq!(multiset.count(&20), 3);
}

#[test]
fn test_random_insert() {
    const INSERT_SIZE: usize = 10_000;

    let mut map = BTreeMap::<u8, usize>::new();
    let mut multiset = MultiSet::<u8>::new();

    for _ in 0..INSERT_SIZE {
        let x = random();

        // mapに追加
        *map.entry(x).or_insert(0) += 1;

        // multisetに挿入
        multiset.insert(x);

        assert_eq!(map[&x], multiset.count(&x));
    }
}

#[test]
fn test_random_insert_delete() {
    const QUERY_SIZE: usize = 10_000;

    let mut map = BTreeMap::<u8, usize>::new();
    let mut multiset = MultiSet::<u8>::new();

    for _ in 0..QUERY_SIZE {
        let x = random();

        // mapに追加
        *map.entry(x).or_insert(0) += 1;

        // multisetに挿入
        multiset.insert(x);

        assert_eq!(map[&x], multiset.count(&x));

        let y = random();

        // mapから削除
        map.get_mut(&y).filter(|v| **v > 0).map(|v| *v -= 1);

        // multisetに挿入
        multiset.delete(&y);

        assert_eq!(map.get(&y).unwrap_or(&0), &multiset.count(&y));
    }
}
