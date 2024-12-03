//! 検索のテスト

use btree_simple::{get, get_mut, insert_multi, print_as_tree, NodePtr};

#[test]
fn test_get() {
    let mut tree: Option<NodePtr<2, char, &str>> = None;

    let keys = vec![
        ('a', "Apple"),
        ('o', "Orange"),
        ('k', "Kebab"),
        ('b', "Banana"),
        ('n', "Noodles"),
        ('e', "Elderberry"),
        ('t', "Tomato"),
        ('c', "Cherry"),
        ('f', "Fig"),
        ('z', "Zucchini"),
        ('g', "Grape"),
    ];

    for &(k, v) in keys.iter() {
        tree = insert_multi(tree, k, v);
    }

    print_as_tree(&tree);

    for key in 'a'..='z' {
        let val = get(&tree, &key);

        println!("key:{}, val:{:?}", key, val);

        let ans = keys.iter().find(|(k, _)| &key == k);

        if let Some((_, v)) = ans {
            assert!(val.is_some());
            assert_eq!(v, val.unwrap());
        } else {
            assert!(val.is_none());
        }
    }
}

#[test]
fn test_get_mut() {
    let mut tree: Option<NodePtr<2, char, String>> = None;

    let mut keys = vec![
        ('a', "Apple".to_string()),
        ('o', "Orange".to_string()),
        ('k', "Kebab".to_string()),
        ('b', "Banana".to_string()),
        ('n', "Noodles".to_string()),
        ('e', "Elderberry".to_string()),
        ('t', "Tomato".to_string()),
        ('c', "Cherry".to_string()),
        ('f', "Fig".to_string()),
        ('z', "Zucchini".to_string()),
        ('g', "Grape".to_string()),
    ];

    for (k, v) in &keys {
        tree = insert_multi(tree, *k, v.clone());
    }

    print_as_tree(&tree);

    for key in 'a'..='z' {
        let val = get_mut(&mut tree, &key);

        println!("key:{}, val:{:?}", key, val);

        let ans = keys.iter_mut().find(|(k, _)| &key == k);

        match (val, ans) {
            (Some(res), Some((_, ans))) => {
                assert_eq!(res, ans);

                // 更新
                *res += " Updated";
                *ans += " Updated";
            }
            (None, None) => {}
            _ => {
                panic!("answer and actually is not same");
            }
        }
    }

    print_as_tree(&tree);

    for key in 'a'..='z' {
        let val = get(&tree, &key);

        println!("key:{}, val:{:?}", key, val);

        let ans = keys.iter().find(|(k, _)| &key == k);

        if let Some((_, v)) = ans {
            assert!(val.is_some());
            assert_eq!(v, val.unwrap());
        } else {
            assert!(val.is_none());
        }
    }
}
