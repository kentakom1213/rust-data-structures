//! 削除のテスト

use btree_simple::{
    btree,
    debug::print_as_tree,
    insert::insert,
    node::{BTreeNode, NodePtr},
    remove::{remove, RemoveKey},
};
use rand::Rng;
use rstest::rstest;
use rustc_hash::FxHashMap;

const COUNT: u32 = 200000;

#[test]
fn test_remove_incremental_D2() {
    let mut tree: Option<NodePtr<2, u32, String>> = None;

    for i in 0..COUNT {
        tree = insert(tree, i, i.to_string());
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
        tree = insert(tree, i, i.to_string());
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
        tree = insert(tree, i, i.to_string());
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
        tree = insert(tree, i, i.to_string());
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
}

#[test]
fn test_hand_2() {
    let mut tree: Option<NodePtr<2, u32, ()>> = btree! {
        keys: [Some(7), Some(18), Some(27)],
        vals: [Some(()), Some(()), Some(())],
        children: [
            btree! {
                keys: [Some(1), Some(1), Some(3)],
                vals: [Some(()), Some(()), Some(())],
                size: 3
            },
            btree! {
                keys: [Some(7), Some(17), None],
                vals: [Some(()), Some(()), None],
                size: 2
            },
            btree! {
                keys: [Some(22), None, None],
                vals: [Some(()), None, None],
                size: 1
            },
            btree! {
                keys: [Some(32), Some(43), Some(49)],
                vals: [Some(()), Some(()), Some(())],
                size: 3
            },
        ],
        size: 3,
    };

    print_as_tree(&tree);

    println!("> remove 12");

    let removed;
    (tree, removed) = remove(tree, RemoveKey::Key(&12));

    print_as_tree(&tree);
    assert_eq!(removed.unwrap().0, 12);
}

#[rstest(
    max,
    count,
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    case(30, 20),
    // case(100, 10000),
    // case(1000, 1000),
    // case(1000, 10000),
)]
fn test_random(max: u32, count: u32) {
    let mut rng = rand::thread_rng();
    let mut set = FxHashMap::default();

    let mut tree: Option<NodePtr<2, u32, String>> = None;

    for _ in 0..count {
        let x = rng.gen_range(0..max);
        tree = insert(tree, x, x.to_string());
        *set.entry(x).or_insert(0) += 1;
    }

    // print_as_tree(&tree);

    for _ in 0..count {
        let x = rng.gen_range(0..max);

        // println!("> remove {x}");

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&x));

        // print_as_tree(&tree);

        let tmp = tree.clone();

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
                println!("> remove {}", x);
                print_as_tree(&tmp);
                panic!("answer: {:?}, actually: {:?}", ans, act);
            }
        }
    }
}
