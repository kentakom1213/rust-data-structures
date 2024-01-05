use aa_tree_traversable::{
    node::{AATreeNode, AATreeNodeInner},
    print_util::print_as_btree,
    tree,
};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_tree() {
    let null_tree: AATreeNode<&str, i32> = None;

    print_as_btree(&null_tree);

    let mono_tree = tree! {
        key: "root",
        value: 0,
        level: 1,
    };

    print_as_btree(&mono_tree);

    /*

       |
       B - D
      /   / \
     A   C   E

    */
    let tree = tree! {
        key: "B",
        value: 2,
        level: 2,
        left: tree! {
            key: "A",
            value: 1,
            level: 1,
        },
        right: tree! {
            key: "D",
            value: 4,
            level: 2,
            left: tree! {
                key: "C",
                value: 3,
                level: 1,
            },
            right: tree! {
                key: "E",
                value: 5,
                level: 1,
            }
        }
    };

    print_as_btree(&tree);
}
