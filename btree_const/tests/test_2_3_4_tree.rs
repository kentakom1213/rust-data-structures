use btree_const::{
    btree,
    debug_util::print_as_tree,
    insert::insert,
    node::{Node, NodePtr},
};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_debug_print() {
    // DEG=3（2-3木）
    let tree: Option<NodePtr<u8, &str>> = btree! {
        keys: [Some(5), Some(18), Some(40)],
        vals: [Some("0005"), Some("0018"), Some("0040")],
        children: [
            btree! {
                keys: [Some(1), Some(3), None],
                vals: [Some("0001"), Some("0003"), None],
                size: 2,
            },
            btree! {
                keys: [Some(10), None, None],
                vals: [Some("0010"), None, None],
                children: [
                    btree! {
                        keys: [Some(8), Some(9), None],
                        vals: [Some("0008"), Some("0009"), None],
                        size: 2,
                    },
                    btree! {
                        keys: [Some(12), Some(14), None],
                        vals: [Some("0012"), Some("0014"), None],
                        children: [
                            btree! {
                                keys: [Some(10), Some(11), None],
                                vals: [Some("0010"), Some("0011"), None],
                                size: 2,
                            },
                            btree! {
                                keys: [Some(13), None, None],
                                vals: [Some("0013"), None, None],
                                size: 1,
                            },
                            btree! {
                                keys: [Some(16), Some(17), Some(19)],
                                vals: [Some("0016"), Some("0017"), Some("0019")],
                                size: 3,
                            },
                            btree! {
                                keys: [None, None, None],
                                vals: [None, None, None],
                                size: 0,
                            }
                        ],
                        size: 2,
                    },
                    btree! {
                        keys: [None, None, None],
                        vals: [None, None, None],
                        size: 0
                    },
                    btree! {
                        keys: [None, None, None],
                        vals: [None, None, None],
                        size: 0
                    }
                ],
                size: 1,
            },
            btree! {
                keys: [Some(21), Some(27), Some(30)],
                vals: [Some("0021"), Some("0027"), Some("0030")],
                size: 3,
            },
            btree! {
                keys: [Some(43), Some(51), None],
                vals: [Some("0043"), Some("0051"), None],
                size: 2,
            },
        ],
        size: 3,
    };

    print_as_tree(&tree);
}

/// 空きのあるノードに挿入
#[test]
fn test_insert_with_vacent() {
    let mut tree: Option<NodePtr<u8, &str>> = Some(Node::alloc_leaf());

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
