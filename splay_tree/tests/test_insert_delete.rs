#![cfg(test)]

use splay_tree::splay_tree::*;

#[test]
fn test_insert_random() {
    let mut tree = SplayTree::new();

    // 挿入
    println!("> Insert (3, \"1st\")");
    assert_eq!(tree.insert(3, "1st"), true);
    println!("{:?}", &tree);
    
    println!("> Insert (1, \"2nd\")");
    assert_eq!(tree.insert(1, "2nd"), true);
    println!("{:?}", &tree);
    
    println!("> Insert (4, \"3rd\")");
    assert_eq!(tree.insert(4, "3rd"), true);
    println!("{:?}", &tree);
    
    println!("> Insert (1, \"4th\")");
    assert_eq!(tree.insert(1, "4th"), false);
    println!("{:?}", &tree);
    
    println!("> Insert (5, \"5th\")");
    assert_eq!(tree.insert(5, "5th"), true);
    println!("{:?}", &tree);
    
    println!("> Insert (9, \"6th\")");
    assert_eq!(tree.insert(9, "6th"), true);
    println!("{:?}", &tree);
    
    println!("> Insert (2, \"7th\")");
    assert_eq!(tree.insert(2, "7th"), true);
    println!("{:?}", &tree);
    
    println!("> Insert (6, \"8th\")");
    assert_eq!(tree.insert(6, "8th"), true);
    println!("{:?}", &tree);

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
    println!("{:?}", &tree);

    // 検索
    // assert_eq!(tree.search(&2), false);
    // assert_eq!(tree.search(&6), false);
    // assert_eq!(tree.search(&5), true);
    // assert_eq!(tree.search(&3), true);
    // assert_eq!(tree.search(&5), true);
    // assert_eq!(tree.search(&1), true);
}
