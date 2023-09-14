#![cfg(test)]

use splay_tree::{splay_tree::*, tree};

#[test]
fn test_splay_left() {
    let mut tree = SplayTree::new();

    tree.root = tree! {
        key: 3,
        value: "3",
        left: tree! {
            key: 2,
            value: "2",
            left: tree! {
                key: 1,
                value: "1",
                left: tree! {
                    key: 0,
                    value: "0",
                }
            }
        }
    };

    println!("----- before -----");
    println!("{:?}", &tree);

    tree.splay(&2);

    println!("----- after -----");
    println!("{:?}", &tree);
}

#[test]
fn test_splay_left_left() {
    let mut tree = SplayTree::new();

    tree.root = tree! {
        key: 3,
        value: "3",
        left: tree! {
            key: 2,
            value: "2",
            left: tree! {
                key: 1,
                value: "1",
                left: tree! {
                    key: 0,
                    value: "0",
                }
            }
        }
    };

    println!("----- before -----");
    println!("{:?}", &tree);

    tree.splay(&0);

    println!("----- after -----");
    println!("{:?}", &tree);
}

#[test]
fn test_splay_left_right() {
    let mut tree = SplayTree::new();

    tree.root = tree! {
        key: 2,
        value: "2",
        left: tree! {
            key: 0,
            value: "0",
            right: tree! {
                key: 1,
                value: "1",
            }
        }
    };

    println!("----- before -----");
    println!("{:?}", &tree);

    tree.splay(&1);

    println!("----- after -----");
    println!("{:?}", &tree);
}

#[test]
fn test_from_path() {
    let mut tree: SplayTree<u8, &str> = SplayTree::new();

    tree.root = tree! {
        key: 5,
        value: "alpha",
        left: tree! {
            key: 4,
            value: "beta",
            left: tree! {
                key: 3,
                value: "gamma",
                left: tree! {
                    key: 2,
                    value: "delta",
                    left: tree! {
                        key: 1,
                        value: "epsilon",
                        left: tree! {
                            key: 0,
                            value: "zeta",
                        }
                    }
                }
            }
        }
    };

    println!("### before splay ###");
    println!("{:?}", &tree);

    // スプレー操作を行う
    tree.splay(&0);

    println!("### after splay ###");
    println!("{:?}", &tree);
}
