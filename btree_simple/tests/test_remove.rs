//! 削除のテスト

use btree_simple::{
    debug::print_as_tree,
    insert::insert,
    node::NodePtr,
    remove::{remove, RemoveKey},
};

#[test]
fn test_remove_incremental() {
    let count = 500;

    let mut tree: Option<NodePtr<2, i32, String>> = None;

    for i in 0..count {
        tree = insert(tree, i, i.to_string());
    }

    print_as_tree(&tree);

    for i in 0..count {
        println!("> remove {i}");

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&i));

        print_as_tree(&tree);
        assert_eq!(removed.unwrap().0, i);
    }
}

#[test]
fn test_remove_decremental() {
    let count = 500;

    let mut tree: Option<NodePtr<2, i32, String>> = None;

    for i in 0..count {
        tree = insert(tree, i, i.to_string());
    }

    print_as_tree(&tree);

    for i in (0..count).rev() {
        println!("> remove {i}");

        let removed;
        (tree, removed) = remove(tree, RemoveKey::Key(&i));

        print_as_tree(&tree);
        assert_eq!(removed.unwrap().0, i);
    }
}
