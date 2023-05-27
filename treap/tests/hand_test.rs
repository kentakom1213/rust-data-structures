#![cfg(test)]

use treap::treap::*;
use rand::prelude::*;

#[test]
fn test_insert_and_find() {
    let mut tree = Treap::new();

    // 挿入
    assert_eq!(tree.insert(3), true);
    assert_eq!(tree.insert(1), true);
    assert_eq!(tree.insert(4), true);
    assert_eq!(tree.insert(1), false);
    assert_eq!(tree.insert(5), true);
    assert_eq!(tree.insert(9), true);
    tree.pretty_print();

    // 検索
    assert_eq!(tree.search(&2), false);
    assert_eq!(tree.search(&6), false);
    assert_eq!(tree.search(&5), true);
    assert_eq!(tree.search(&3), true);
    assert_eq!(tree.search(&5), true);
    assert_eq!(tree.search(&1), true);
}

#[test]
fn test_insert_and_delete_1() {
    let mut tree = Treap::new();
    assert_eq!(tree.len(), 0);

    // 挿入
    assert_eq!(tree.insert(3), true);
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.insert(1), true);
    assert_eq!(tree.len(), 2);
    assert_eq!(tree.insert(4), true);
    assert_eq!(tree.len(), 3);
    assert_eq!(tree.insert(1), false);
    assert_eq!(tree.len(), 3);
    assert_eq!(tree.insert(5), true);
    assert_eq!(tree.len(), 4);
    assert_eq!(tree.insert(9), true);
    assert_eq!(tree.len(), 5);

    // 100(存在しない値)を削除
    assert_eq!(tree.discard(&100), false);
    assert_eq!(tree.len(), 5);

    // 表示
    println!("\n##### test_insert_and_delete_1 #####");
    tree.pretty_print();

    // 9(葉)を削除
    println!("----- delete 9 -----");
    assert_eq!(tree.discard(&9), true);
    assert_eq!(tree.search(&9), false);
    assert_eq!(tree.len(), 4);
    tree.pretty_print();

    // 4(子1つ)を削除
    println!("----- delete 4 -----");
    assert_eq!(tree.discard(&4), true);
    assert_eq!(tree.search(&4), false);
    assert_eq!(tree.len(), 3);
    tree.pretty_print();
}

#[test]
fn test_insert_and_delete_2() {
    let mut tree = Treap::new();

    // 挿入
    tree.insert(25);
    tree.insert(20);
    tree.insert(30);
    tree.insert(15);
    tree.insert(23);
    tree.insert(10);
    tree.insert(18);
    tree.insert(16);

    // 表示
    println!("\n##### test_insert_and_delete_2 #####");
    tree.pretty_print();

    // 18(子2つ)を削除
    println!("----- delete 20 -----");
    assert_eq!(tree.discard(&20), true);
    assert_eq!(tree.search(&20), false);
    tree.pretty_print();

    // 15(子2つ)を削除
    println!("----- delete 15 -----");
    assert_eq!(tree.discard(&15), true);
    assert_eq!(tree.search(&15), false);
    tree.pretty_print();

    // 18(子2つ)を削除
    println!("----- delete 18 -----");
    assert_eq!(tree.discard(&18), true);
    assert_eq!(tree.search(&18), false);
    tree.pretty_print();

    // 25(子2つ)を削除
    println!("----- delete 25 -----");
    assert_eq!(tree.discard(&25), true);
    assert_eq!(tree.search(&25), false);
    tree.pretty_print();
}

#[test]
fn test_lower_bound() {
    let mut tree = Treap::new();
    tree.insert(1);
    tree.insert(3);
    tree.insert(5);
    tree.insert(8);

    tree.pretty_print();

    // lower_bound
    assert_eq!(tree.lower_bound(&0), Some(&1));
    assert_eq!(tree.lower_bound(&1), Some(&1));
    assert_eq!(tree.lower_bound(&3), Some(&3));
    assert_eq!(tree.lower_bound(&7), Some(&8));
    assert_eq!(tree.lower_bound(&10), None);
}

#[test]
fn test_upper_bound() {
    let mut tree = Treap::new();
    tree.insert(1);
    tree.insert(3);
    tree.insert(5);
    tree.insert(8);

    tree.pretty_print();

    // lower_bound
    assert_eq!(tree.upper_bound(&0), Some(&1));
    assert_eq!(tree.upper_bound(&1), Some(&3));
    assert_eq!(tree.upper_bound(&3), Some(&5));
    assert_eq!(tree.upper_bound(&7), Some(&8));
    assert_eq!(tree.upper_bound(&10), None);
}
