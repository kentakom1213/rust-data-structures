//! 削除のテスト

use btree_simple::{
    debug::print_as_tree,
    insert::insert,
    node::NodePtr,
    remove::{remove, RemoveKey},
};
use rand::{random, Rng};
use rstest::rstest;
use rustc_hash::{FxHashMap, FxHashSet};

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

#[rstest(max, count, case(100, 10))]
fn test_random(max: u32, count: u32) {
    let mut rng = rand::thread_rng();
    let mut set = FxHashMap::default();

    let mut tree: Option<NodePtr<2, u32, String>> = None;

    for _ in 0..count {
        let x = rng.gen_range(0..max);
        tree = insert(tree, x, x.to_string());
        *set.entry(x).or_insert(0) += 1;
    }

    print_as_tree(&tree);

    for _ in 0..count {
        let x = rng.gen_range(0..max);

        println!("> remove {x}");

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&x));

        print_as_tree(&tree);

        match (set.get_mut(&x), removed) {
            (Some(cnt), Some((key, _))) => {
                assert_eq!(key, x);
                *cnt -= 1;
                if *cnt == 0 {
                    set.remove(&x);
                }
            }
            (None, None) => {}
            _ => panic!("unexpected"),
        }
    }
}
