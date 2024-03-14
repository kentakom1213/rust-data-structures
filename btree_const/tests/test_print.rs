use btree_const::{btree, debug_util::dbg_inner, node::Node};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[test]
fn test_debug_print() {
    // DEG=3（2-3木）
    let tree = btree! {
        keys: [Some(0), Some(5)],
        vals: [Some("0003"), Some("0005")],
        children: [
            btree! {
                keys: [Some(1), Some(2)],
                vals: [Some("0001"), Some("0002")],
                len: 2,
            },
            btree! {
                keys: [Some(4), None],
                vals: [Some("0004"), None],
                len: 1,
            },
            btree! {
                keys: [Some(6), None],
                vals: [Some("0006"), None],
                len: 1,
            },
        ],
        len: 2,
    };

    dbg_inner(&tree, 0);
}
