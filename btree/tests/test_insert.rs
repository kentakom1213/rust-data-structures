#![feature(generic_const_exprs)]

use btree::{
    debug_util::print_as_tree,
    insert::insert,
    node::{Node, NodePtr},
};

/// 空きのあるノードに挿入
#[test]
fn test_insert_with_vacent() {
    let mut tree: Option<NodePtr<3, u8, &str>> = Some(Node::alloc_leaf());

    // 空きがある
    assert_eq!(tree.as_ref().unwrap().borrow().has_vacant(), true);

    eprintln!("\n> insert 20");
    tree = insert(tree, 20, "0020");
    eprintln!("{:?}", &tree);
    print_as_tree(&tree);

    // 空きがある
    assert_eq!(tree.as_ref().unwrap().borrow().has_vacant(), true);

    eprintln!("\n> insert 50");
    tree = insert(tree, 50, "0050");
    eprintln!("{:?}", &tree);
    print_as_tree(&tree);

    // 空きがある
    assert_eq!(tree.as_ref().unwrap().borrow().has_vacant(), true);

    eprintln!("\n> insert 10");
    tree = insert(tree, 10, "0010");
    eprintln!("{:?}", &tree);
    print_as_tree(&tree);

    // → 満杯になる
    assert_eq!(tree.as_ref().unwrap().borrow().has_vacant(), false);
}
