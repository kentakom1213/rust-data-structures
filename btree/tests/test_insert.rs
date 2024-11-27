#![feature(generic_const_exprs)]

use btree::{
    debug_util::print_as_tree,
    insert::insert,
    node::{BTreeNode, NodePtr},
    node_util::NodeUtil,
};

/// 空きのあるノードに挿入
#[test]
fn test_insert_with_vacent() {
    let mut tree: NodePtr<2, i32, &str> = BTreeNode::alloc_leaf();

    // 空きがある
    assert_eq!(tree.is_full(), false);

    eprintln!("\n> insert 20");
    tree = insert(tree, 20, "0020");
    print_as_tree(&Some(tree.clone()));

    // 空きがある
    assert_eq!(tree.is_full(), false);

    eprintln!("\n> insert 50");
    tree = insert(tree, 50, "0050");
    print_as_tree(&Some(tree.clone()));

    // 空きがある
    assert_eq!(tree.is_full(), false);

    eprintln!("\n> insert 10");
    tree = insert(tree, 10, "0010");
    print_as_tree(&Some(tree.clone()));

    // → 満杯になる
    assert_eq!(tree.is_full(), true);

    // さらに挿入すると分割される
    eprintln!("\n> insert 30");
    tree = insert(tree, 30, "0030");
    print_as_tree(&Some(tree.clone()));

    // 挿入
    eprintln!("\n> insert 20");
    tree = insert(tree, 20, "0020(2)");
    print_as_tree(&Some(tree.clone()));

    // 挿入
    eprintln!("\n> insert 15");
    tree = insert(tree, 15, "0015");
    print_as_tree(&Some(tree.clone()));

    // 挿入
    eprintln!("\n> insert 13");
    tree = insert(tree, 13, "0013");
    print_as_tree(&Some(tree.clone()));

    // 挿入
    eprintln!("\n> insert 24");
    tree = insert(tree, 24, "0024");
    print_as_tree(&Some(tree.clone()));

    // 挿入
    eprintln!("\n> insert 39");
    tree = insert(tree, 39, "0039");
    print_as_tree(&Some(tree.clone()));

    // 挿入
    eprintln!("\n> insert 100");
    tree = insert(tree, 100, "0100");
    print_as_tree(&Some(tree.clone()));

    // 挿入
    eprintln!("\n> insert 400");
    tree = insert(tree, 400, "0400");
    print_as_tree(&Some(tree.clone()));

    // 挿入
    eprintln!("\n> insert 23");
    tree = insert(tree, 23, "0023");
    print_as_tree(&Some(tree.clone()));

    // 挿入
    eprintln!("\n> insert 22");
    tree = insert(tree, 22, "0022");
    print_as_tree(&Some(tree.clone()));
}
