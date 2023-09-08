#![cfg(test)]

use splay_tree::splay_tree::SplayTree;

#[test]
fn test_insert_random() {
    let mut tree = SplayTree::new();

    // 挿入
    assert_eq!(tree.insert(3, "1st"), true);
    assert_eq!(tree.insert(1, "2nd"), true);
    assert_eq!(tree.insert(4, "3rd"), true);
    assert_eq!(tree.insert(1, "4th"), false);
    assert_eq!(tree.insert(5, "5th"), true);
    assert_eq!(tree.insert(9, "6th"), true);
    assert_eq!(tree.insert(2, "7th"), true);
    assert_eq!(tree.insert(6, "8th"), true);
    tree.pretty_print();

    // 検索
    // assert_eq!(tree.search(&2), false);
    // assert_eq!(tree.search(&6), false);
    // assert_eq!(tree.search(&5), true);
    // assert_eq!(tree.search(&3), true);
    // assert_eq!(tree.search(&5), true);
    // assert_eq!(tree.search(&1), true);
}

#[test]
fn test_insert_sorted() {
    let mut tree = SplayTree::new();

    // 挿入
    assert_eq!(tree.insert(1, "1st"), true);
    assert_eq!(tree.insert(2, "2nd"), true);
    assert_eq!(tree.insert(3, "3rd"), true);
    assert_eq!(tree.insert(4, "4th"), true);
    assert_eq!(tree.insert(5, "5th"), true);
    assert_eq!(tree.insert(6, "6th"), true);
    assert_eq!(tree.insert(7, "7th"), true);
    assert_eq!(tree.insert(8, "8th"), true);
    tree.pretty_print();

    // 検索
    // assert_eq!(tree.search(&2), false);
    // assert_eq!(tree.search(&6), false);
    // assert_eq!(tree.search(&5), true);
    // assert_eq!(tree.search(&3), true);
    // assert_eq!(tree.search(&5), true);
    // assert_eq!(tree.search(&1), true);
}

