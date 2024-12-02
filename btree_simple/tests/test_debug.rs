use btree_simple::{
    btree,
    debug::print_as_tree,
    node::{BTreeNode, NodePtr},
};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_debug_print() {
    // DEG=3（2-3木）
    let tree: Option<NodePtr<2, u8, &str>> = btree! {
        keys: [Some(5), Some(25), Some(40)],
        vals: [Some("0005"), Some("0025"), Some("0040")],
        children: [
            btree! {
                keys: [Some(1), None, None],
                vals: [Some("0001"), None, None],
                children: [
                    btree! {
                        keys: [Some(0), None, None],
                        vals: [Some("0000"), None, None],
                        size: 1,
                    },
                    btree! {
                        keys: [Some(3), None, None],
                        vals: [Some("0003"), None, None],
                        size: 1,
                    },
                    btree! {
                        keys: [None, None, None],
                        vals: [None, None, None],
                        size: 0,
                    },
                    btree! {
                        keys: [None, None, None],
                        vals: [None, None, None],
                        size: 0,
                    }
                ],
                size: 1,
            },
            btree! {
                keys: [Some(10), Some(20), None],
                vals: [Some("0010"), Some("0020"), None],
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
                                keys: [Some(11), None, None],
                                vals: [Some("0011"), None, None],
                                size: 1,
                            },
                            btree! {
                                keys: [Some(13), None, None],
                                vals: [Some("0013"), None, None],
                                size: 1,
                            },
                            btree! {
                                keys: [Some(15), Some(16), Some(17)],
                                vals: [Some("0015"), Some("0016"), Some("0017")],
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
                        keys: [Some(19), Some(23), None],
                        vals: [Some("0019"), Some("0023"), None],
                        size: 2,
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
                keys: [Some(27), Some(30), None],
                vals: [Some("0027"), Some("0030"), None],
                size: 2,
            },
            btree! {
                keys: [Some(51), None, None],
                vals: [Some("0051"), None, None],
                children: [
                    btree! {
                        keys: [Some(45), None, None],
                        vals: [Some("0045"), None, None],
                        children: [
                            btree! {
                                keys: [Some(41), None, None],
                                vals: [Some("0041"), None, None],
                                size: 1,
                            },
                            btree! {
                                keys: [Some(47), None, None],
                                vals: [Some("0047"), None, None],
                                size: 1,
                            },
                            btree! {
                                keys: [None, None, None],
                                vals: [None, None, None],
                                size: 0,
                            },
                            btree! {
                                keys: [None, None, None],
                                vals: [None, None, None],
                                size: 0,
                            }
                        ],
                        size: 1,
                    },
                    btree! {
                        keys: [Some(55), None, None],
                        vals: [Some("0055"), None, None],
                        children: [
                            btree! {
                                keys: [Some(53), None, None],
                                vals: [Some("0053"), None, None],
                                size: 1,
                            },
                            btree! {
                                keys: [Some(57), None, None],
                                vals: [Some("0057"), None, None],
                                size: 1,
                            },
                            btree! {
                                keys: [None, None, None],
                                vals: [None, None, None],
                                size: 0,
                            },
                            btree! {
                                keys: [None, None, None],
                                vals: [None, None, None],
                                size: 0,
                            }
                        ],
                        size: 1,
                    },
                    btree! {
                        keys: [None, None, None],
                        vals: [None, None, None],
                        size: 0,
                    },
                    btree! {
                        keys: [None, None, None],
                        vals: [None, None, None],
                        size: 0,
                    }
                ],
                size: 1,
            },
        ],
        size: 3,
    };

    print_as_tree(&tree);
}
