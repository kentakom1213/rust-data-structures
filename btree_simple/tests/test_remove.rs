//! 削除のテスト

#![allow(non_snake_case)]

use btree_simple::{btree, insert_multi, print_as_tree, remove, BTreeNode, NodePtr, RemoveKey};
use rand::Rng;
use rstest::rstest;
use rustc_hash::FxHashMap;

const COUNT: u32 = 200000;

#[test]
fn test_remove_incremental_D2() {
    let mut tree: Option<NodePtr<2, u32, String>> = None;

    for i in 0..COUNT {
        tree = insert_multi(tree, i, i.to_string());
    }

    // print_as_tree(&tree);

    for i in 0..COUNT {
        // println!("> remove {i}");

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&i));

        // print_as_tree(&tree);
        assert_eq!(removed.unwrap().0, i);
    }
}

#[test]
fn test_remove_decremental_D2() {
    let mut tree: Option<NodePtr<2, u32, String>> = None;

    for i in 0..COUNT {
        tree = insert_multi(tree, i, i.to_string());
    }

    // print_as_tree(&tree);

    for i in (0..COUNT).rev() {
        // println!("> remove {i}");

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&i));

        // print_as_tree(&tree);
        assert_eq!(removed.unwrap().0, i);
    }
}

#[test]
fn test_remove_incremental_D3() {
    let mut tree: Option<NodePtr<3, u32, String>> = None;

    for i in 0..COUNT {
        tree = insert_multi(tree, i, i.to_string());
    }

    // print_as_tree(&tree);

    for i in 0..COUNT {
        // println!("> remove {i}");

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&i));

        // print_as_tree(&tree);
        assert_eq!(removed.unwrap().0, i);
    }
}

#[test]
fn test_remove_decremental_D3() {
    let mut tree: Option<NodePtr<3, u32, String>> = None;

    for i in 0..COUNT {
        tree = insert_multi(tree, i, i.to_string());
    }

    // print_as_tree(&tree);

    for i in (0..COUNT).rev() {
        // println!("> remove {i}");

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&i));

        // print_as_tree(&tree);
        assert_eq!(removed.unwrap().0, i);
    }
}

#[test]
fn test_hand_1() {
    let mut tree: Option<NodePtr<2, i32, ()>> = btree! {
        keys: [Some(31), Some(67), None],
        vals: [Some(()), Some(()), None],
        children: [
            btree! {
                keys: [Some(0), Some(4), Some(11)],
                vals: [Some(()), Some(()), Some(())],
                size: 3
            },
            btree! {
                keys: [Some(35), Some(37), Some(55)],
                vals: [Some(()), Some(()), Some(())],
                size: 3
            },
            btree! {
                keys: [Some(81), Some(90), None],
                vals: [Some(()), Some(()), None],
                size: 2
            },
            None,
        ],
        size: 2
    };

    print_as_tree(&tree);

    let removed;
    (tree, removed) = remove(tree, RemoveKey::Key(&67));

    print_as_tree(&tree);
    assert_eq!(removed.unwrap().0, 67);
}

#[test]
fn test_hand_2() {
    let mut tree: Option<NodePtr<2, u32, ()>> = btree! {
        keys: [Some(11), None, None],
        vals: [Some(()), None, None],
        children: [
            btree! {
                keys: [Some(8), None, None],
                vals: [Some(()), None, None],
                children: [
                    btree! {
                        keys: [Some(4), Some(7), None],
                        vals: [Some(()), Some(()), None],
                        size: 2
                    },
                    btree! {
                        keys: [Some(9), Some(10), None],
                        vals: [Some(()), Some(()), None],
                        size: 2
                    },
                    None,
                    None,
                ],
                size: 1
            },
            btree! {
                keys: [Some(18), None, None],
                vals: [Some(()), None, None],
                children: [
                    btree! {
                        keys: [Some(14), Some(15), None],
                        vals: [Some(()), Some(()), None],
                        size: 2
                    },
                    btree! {
                        keys: [Some(18), Some(20), Some(20)],
                        vals: [Some(()), Some(()), Some(())],
                        size: 3
                    },
                    None,
                    None,
                ],
                size: 1
            },
            None,
            None,
        ],
        size: 1,
    };

    let rm = 11;

    print_as_tree(&tree);

    println!("> remove {rm}");

    let removed;
    (tree, removed) = remove(tree, RemoveKey::Key(&rm));

    print_as_tree(&tree);
    assert_eq!(removed.unwrap().0, rm);
}

#[rstest(
    max,
    count,
    case(30, 100),
    case(30, 100),
    case(30, 100),
    case(30, 100),
    case(30, 100),
    case(100, 1000),
    case(100, 1000),
    case(100, 1000),
    case(100, 1000),
    case(100, 1000),
    case(100, 10000),
    case(100, 10000),
    case(100, 10000),
    case(1000, 1000),
    case(1000, 1000),
    case(1000, 1000),
    case(10000, 10000),
    case(10000, 10000),
    case(10000, 10000)
)]
fn test_random_D2(max: u32, count: u32) {
    let mut rng = rand::thread_rng();
    let mut set = FxHashMap::default();

    let mut tree: Option<NodePtr<2, u32, String>> = None;

    for _ in 0..count {
        let x = rng.gen_range(0..max);
        tree = insert_multi(tree, x, x.to_string());
        *set.entry(x).or_insert(0) += 1;
    }

    for _ in 0..count {
        let x = rng.gen_range(0..max);

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&x));

        match (set.get_mut(&x), removed) {
            (Some(cnt), Some((key, _))) => {
                assert_eq!(key, x);
                *cnt -= 1;
                if *cnt == 0 {
                    set.remove(&x);
                }
            }
            (None, None) => {}
            (ans @ _, act @ _) => {
                panic!("answer: {:?}, actually: {:?}", ans, act);
            }
        }
    }
}

#[rstest(
    max,
    count,
    case(30, 100),
    case(30, 100),
    case(30, 100),
    case(30, 100),
    case(30, 100),
    case(100, 1000),
    case(100, 1000),
    case(100, 1000),
    case(100, 1000),
    case(100, 1000),
    case(100, 10000),
    case(100, 10000),
    case(100, 10000),
    case(1000, 1000),
    case(1000, 1000),
    case(1000, 1000),
    case(10000, 10000),
    case(10000, 10000),
    case(10000, 10000)
)]
fn test_random_D3(max: u32, count: u32) {
    let mut rng = rand::thread_rng();
    let mut set = FxHashMap::default();

    let mut tree: Option<NodePtr<3, u32, String>> = None;

    for _ in 0..count {
        let x = rng.gen_range(0..max);
        tree = insert_multi(tree, x, x.to_string());
        *set.entry(x).or_insert(0) += 1;
    }

    for _ in 0..count {
        let x = rng.gen_range(0..max);

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&x));

        match (set.get_mut(&x), removed) {
            (Some(cnt), Some((key, _))) => {
                assert_eq!(key, x);
                *cnt -= 1;
                if *cnt == 0 {
                    set.remove(&x);
                }
            }
            (None, None) => {}
            (ans @ _, act @ _) => {
                panic!("answer: {:?}, actually: {:?}", ans, act);
            }
        }
    }
}

#[rstest(
    max,
    count,
    case(30, 100),
    case(30, 100),
    case(30, 100),
    case(30, 100),
    case(30, 100),
    case(100, 1000),
    case(100, 1000),
    case(100, 1000),
    case(100, 1000),
    case(100, 1000),
    case(100, 10000),
    case(100, 10000),
    case(100, 10000),
    case(1000, 1000),
    case(1000, 1000),
    case(1000, 1000),
    case(10000, 10000),
    case(10000, 10000),
    case(10000, 10000)
)]
fn test_random_D4(max: u32, count: u32) {
    let mut rng = rand::thread_rng();
    let mut set = FxHashMap::default();

    let mut tree: Option<NodePtr<4, u32, String>> = None;

    for _ in 0..count {
        let x = rng.gen_range(0..max);
        tree = insert_multi(tree, x, x.to_string());
        *set.entry(x).or_insert(0) += 1;
    }

    for _ in 0..count {
        let x = rng.gen_range(0..max);

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&x));

        match (set.get_mut(&x), removed) {
            (Some(cnt), Some((key, _))) => {
                assert_eq!(key, x);
                *cnt -= 1;
                if *cnt == 0 {
                    set.remove(&x);
                }
            }
            (None, None) => {}
            (ans @ _, act @ _) => {
                panic!("answer: {:?}, actually: {:?}", ans, act);
            }
        }
    }
}
