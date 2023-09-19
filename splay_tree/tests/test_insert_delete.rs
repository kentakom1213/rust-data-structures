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
    println!("> Get 2");
    assert_eq!(tree.get(&2), Some(&"7th"));
    println!("{:?}", &tree);

    println!("> Get 6");
    assert_eq!(tree.get(&6), Some(&"8th"));
    println!("{:?}", &tree);

    println!("> Get 5");
    println!("{:?}", &tree);
    assert_eq!(tree.get(&5), Some(&"5th"));

    println!("> Get 3");
    assert_eq!(tree.get(&3), Some(&"1st"));
    println!("{:?}", &tree);

    println!("> Get 5");
    assert_eq!(tree.get(&5), Some(&"5th"));
    println!("{:?}", &tree);

    println!("> Get 1");
    assert_eq!(tree.get(&1), Some(&"2nd"));
    println!("{:?}", &tree);

    println!("> Get 100");
    assert_eq!(tree.get(&100), None);
    println!("{:?}", &tree);

    println!("> Get 0");
    assert_eq!(tree.get(&0), None);
    println!("{:?}", &tree);
}

#[test]
fn test_insert_sorted() {
    let mut tree = SplayTree::new();

    // 挿入
    println!("> Insert (\"1st\", 3)");
    assert_eq!(tree.insert("1st", 3), true);
    println!("{:?}", &tree);

    println!("> Insert (\"2nd\", 1)");
    assert_eq!(tree.insert("2nd", 1), true);
    println!("{:?}", &tree);

    println!("> Insert (\"3rd\", 4)");
    assert_eq!(tree.insert("3rd", 4), true);
    println!("{:?}", &tree);

    println!("> Insert (\"4th\", 1)");
    assert_eq!(tree.insert("4th", 1), true);
    println!("{:?}", &tree);

    println!("> Insert (\"5th\", 5)");
    assert_eq!(tree.insert("5th", 5), true);
    println!("{:?}", &tree);

    println!("> Insert (\"6th\", 9)");
    assert_eq!(tree.insert("6th", 9), true);
    println!("{:?}", &tree);

    println!("> Insert (\"7th\", 2)");
    assert_eq!(tree.insert("7th", 2), true);
    println!("{:?}", &tree);

    println!("> Insert (\"8th\", 6)");
    assert_eq!(tree.insert("8th", 6), true);
    println!("{:?}", &tree);

    // 検索
    println!("> Get \"7th\"");
    assert_eq!(tree.get(&"7th"), Some(&2));
    println!("{:?}", &tree);

    println!("> Get \"8th\"");
    assert_eq!(tree.get(&"8th"), Some(&6));
    println!("{:?}", &tree);

    println!("> Get \"5th\"");
    assert_eq!(tree.get(&"5th"), Some(&5));
    println!("{:?}", &tree);

    println!("> Get \"1st\"");
    assert_eq!(tree.get(&"1st"), Some(&3));
    println!("{:?}", &tree);

    println!("> Get \"5th\"");
    assert_eq!(tree.get(&"5th"), Some(&5));
    println!("{:?}", &tree);

    println!("> Get \"2nd\"");
    assert_eq!(tree.get(&"2nd"), Some(&1));
    println!("{:?}", &tree);

    println!("> Get \"tooth\"");
    assert_eq!(tree.get(&"tooth"), None);
    println!("{:?}", &tree);

    println!("> Get \"\"");
    assert_eq!(tree.get(&""), None);
    println!("{:?}", &tree);
}
