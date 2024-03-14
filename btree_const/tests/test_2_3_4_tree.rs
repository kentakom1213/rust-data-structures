use btree_const::{
    btree,
    debug_util::dbg_inner,
    insert::insert,
    node::{BTreeNode, Node},
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[test]
fn test_debug_print() {
    // DEG=3（2-3木）
    let tree: BTreeNode<u8, &str> = btree! {
        keys: [Some(5), Some(18), Some(40)],
        vals: [Some("0005"), Some("0018"), Some("0040")],
        children: [
            btree! {
                keys: [Some(1), Some(3), None],
                vals: [Some("0001"), Some("0003"), None],
                len: 2,
            },
            btree! {
                keys: [Some(10), None, None],
                vals: [Some("0010"), None, None],
                len: 1,
            },
            btree! {
                keys: [Some(21), Some(27), Some(30)],
                vals: [Some("0021"), Some("0027"), Some("0030")],
                len: 3,
            },
            btree! {
                keys: [Some(43), Some(51), None],
                vals: [Some("0043"), Some("0051"), None],
                len: 2,
            }
        ],
        len: 3,
    };

    dbg_inner(&tree, 0);
}

/// 空きのあるノードに挿入
#[test]
fn test_insert_with_vacent() {
    let mut tree: BTreeNode<u8, &str> = Node::alloc_leaf();

    // 空きがある
    assert_eq!(tree.as_ref().unwrap().borrow().has_vacant(), true);

    eprintln!("\n> insert 20");
    tree = insert(tree, 20, "0020");
    eprintln!("{:?}", &tree);
    dbg_inner(&tree, 0);

    // 空きがある
    assert_eq!(tree.as_ref().unwrap().borrow().has_vacant(), true);

    eprintln!("\n> insert 50");
    tree = insert(tree, 50, "0050");
    eprintln!("{:?}", &tree);
    dbg_inner(&tree, 0);

    // 空きがある
    assert_eq!(tree.as_ref().unwrap().borrow().has_vacant(), true);

    eprintln!("\n> insert 10");
    tree = insert(tree, 10, "0010");
    eprintln!("{:?}", &tree);
    dbg_inner(&tree, 0);

    // → 満杯になる
    assert_eq!(tree.as_ref().unwrap().borrow().has_vacant(), false);
}
