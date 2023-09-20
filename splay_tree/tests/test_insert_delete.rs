#![cfg(test)]

use splay_tree::splay_tree::*;

#[test]
fn test_insert_random() {
    let mut tree = SplayTree::new();

    // 挿入
    println!("> Insert (3, \"1st\")");
    assert_eq!(tree.insert(3, "1st"), None);
    println!("{:?}", &tree);

    println!("> Insert (1, \"2nd\")");
    assert_eq!(tree.insert(1, "2nd"), None);
    println!("{:?}", &tree);

    println!("> Insert (4, \"3rd\")");
    assert_eq!(tree.insert(4, "3rd"), None);
    println!("{:?}", &tree);

    println!("> Insert (1, \"4th\")");
    assert_eq!(tree.insert(1, "4th"), Some("2nd"));
    println!("{:?}", &tree);

    println!("> Insert (5, \"5th\")");
    assert_eq!(tree.insert(5, "5th"), None);
    println!("{:?}", &tree);

    println!("> Insert (9, \"6th\")");
    assert_eq!(tree.insert(9, "6th"), None);
    println!("{:?}", &tree);

    println!("> Insert (2, \"7th\")");
    assert_eq!(tree.insert(2, "7th"), None);
    println!("{:?}", &tree);

    println!("> Insert (6, \"8th\")");
    assert_eq!(tree.insert(6, "8th"), None);
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
    assert_eq!(tree.get(&1), Some(&"4th"));
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
    assert_eq!(tree.insert("1st", 3), None);
    println!("{:?}", &tree);

    println!("> Insert (\"2nd\", 1)");
    assert_eq!(tree.insert("2nd", 1), None);
    println!("{:?}", &tree);

    println!("> Insert (\"3rd\", 4)");
    assert_eq!(tree.insert("3rd", 4), None);
    println!("{:?}", &tree);

    println!("> Insert (\"4th\", 1)");
    assert_eq!(tree.insert("4th", 1), None);
    println!("{:?}", &tree);

    println!("> Insert (\"5th\", 5)");
    assert_eq!(tree.insert("5th", 5), None);
    println!("{:?}", &tree);

    println!("> Insert (\"6th\", 9)");
    assert_eq!(tree.insert("6th", 9), None);
    println!("{:?}", &tree);

    println!("> Insert (\"7th\", 2)");
    assert_eq!(tree.insert("7th", 2), None);
    println!("{:?}", &tree);

    println!("> Insert (\"8th\", 6)");
    assert_eq!(tree.insert("8th", 6), None);
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

#[test]
fn test_delete() {
    let mut tree = SplayTree::new();

    for i in 1..=20 {
        match i % 10 {
            1 => tree.insert(i, format!("{}st", i)),
            2 => tree.insert(i, format!("{}nd", i)),
            3 => tree.insert(i, format!("{}rd", i)),
            _ => tree.insert(i, format!("{}th", i)),
        };
    }

    println!("{:?}", &tree);

    println!("> Delete 1");
    assert_eq!(tree.delete(&1), Some("1st".to_string()));
    println!("{:?}", &tree);

    println!("> Get 18");
    tree.get(&18);
    println!("{:?}", &tree);

    println!("> Delete 18");
    assert_eq!(tree.delete(&18), Some("18th".to_string()));
    println!("{:?}", &tree);

    println!("> Delete 100");
    assert_eq!(tree.delete(&100), None);
    println!("{:?}", &tree);

    println!("> Delete 2");
    assert_eq!(tree.delete(&2), Some("2nd".to_string()));
    println!("{:?}", &tree);

    println!("> Delete 3");
    assert_eq!(tree.delete(&3), Some("3rd".to_string()));
    println!("{:?}", &tree);

    println!("> Delete 11");
    assert_eq!(tree.delete(&11), Some("11st".to_string()));
    println!("{:?}", &tree);

    println!("> Delete 11");
    assert_eq!(tree.delete(&11), None);
    println!("{:?}", &tree);

    println!("> Delete 11");
    assert_eq!(tree.delete(&11), None);
    println!("{:?}", &tree);
}
