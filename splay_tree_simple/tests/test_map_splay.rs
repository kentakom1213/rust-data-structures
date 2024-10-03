#![cfg(test)]

use splay_tree_simple::{encode::Encode, map::*, tree_map};

#[test]
fn test_splay_left() {
    let mut tree = SplayTreeMap::new();

    tree.root = tree_map! {
        key: 3,
        value: "3",
        left: tree_map! {
            key: 2,
            value: "2",
            left: tree_map! {
                key: 1,
                value: "1",
                left: tree_map! {
                    key: 0,
                    value: "0",
                }
            }
        }
    };

    println!("----- before -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "((((0)1)2)3)");

    tree.splay(&2);

    println!("{}", tree.encode());

    println!("----- after -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "(((0)1)2(3))");
}

#[test]
fn test_splay_left_left() {
    let mut tree = SplayTreeMap::new();

    tree.root = tree_map! {
        key: 3,
        value: "3",
        left: tree_map! {
            key: 2,
            value: "2",
            left: tree_map! {
                key: 1,
                value: "1",
                left: tree_map! {
                    key: 0,
                    value: "0",
                }
            }
        }
    };

    println!("----- before -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "((((0)1)2)3)");

    tree.splay(&0);

    println!("----- after -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "(0((1)2(3)))");
}

#[test]
fn test_splay_right() {
    let mut tree = SplayTreeMap::new();

    tree.root = tree_map! {
        key: 0,
        value: 0,
        right: tree_map! {
            key: 1,
            value: 1,
            right: tree_map! {
                key: 2,
                value: 2
            }
        }
    };

    println!("----- before -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "(0(1(2)))");

    tree.splay(&1);

    println!("----- after -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "((0)1(2))");
}

#[test]
fn test_splay_right_left() {
    let mut tree = SplayTreeMap::new();

    tree.root = tree_map! {
        key: 0,
        value: 0,
        right: tree_map! {
            key: 2,
            value: 2,
            left: tree_map! {
                key: 1,
                value: 1
            }
        }
    };

    println!("----- before -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "(0((1)2))");

    tree.splay(&1);

    println!("----- after -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "((0)1(2))");
}

#[test]
fn test_splay_right_right() {
    let mut tree = SplayTreeMap::new();

    tree.root = tree_map! {
        key: 0,
        value: 0,
        right: tree_map! {
            key: 1,
            value: 1,
            right: tree_map! {
                key: 3,
                value: 3,
                left: tree_map! {
                    key: 2,
                    value: 2,
                }
            }
        }
    };

    println!("----- before -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "(0(1((2)3)))");

    tree.splay(&3);

    println!("----- after -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "(((0)1(2))3)");
}

#[test]
fn test_splay_left_right() {
    let mut tree = SplayTreeMap::new();

    tree.root = tree_map! {
        key: 2,
        value: "2",
        left: tree_map! {
            key: 0,
            value: "0",
            right: tree_map! {
                key: 1,
                value: "1",
            }
        }
    };

    println!("----- before -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "((0(1))2)");

    tree.splay(&1);

    println!("----- after -----");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "((0)1(2))");
}

#[test]
fn test_from_path_left() {
    let mut tree: SplayTreeMap<u8, &str> = SplayTreeMap::new();

    tree.root = tree_map! {
        key: 5,
        value: "alpha",
        left: tree_map! {
            key: 4,
            value: "beta",
            left: tree_map! {
                key: 3,
                value: "gamma",
                left: tree_map! {
                    key: 2,
                    value: "delta",
                    left: tree_map! {
                        key: 1,
                        value: "epsilon",
                        left: tree_map! {
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
    assert_eq!(&tree.encode(), "((((((0)1)2)3)4)5)");

    // スプレー操作を行う
    tree.splay(&0);

    println!("### after splay ###");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "(0(((1)2(3))4(5)))");
}

#[test]
fn test_from_path_right() {
    let mut tree: SplayTreeMap<u8, &str> = SplayTreeMap::new();

    tree.root = tree_map! {
        key: 0,
        value: "alpha",
        right: tree_map! {
            key: 1,
            value: "beta",
            right: tree_map! {
                key: 2,
                value: "gamma",
                right: tree_map! {
                    key: 3,
                    value: "delta",
                    right: tree_map! {
                        key: 4,
                        value: "epsilon",
                        right: tree_map! {
                            key: 5,
                            value: "zeta",
                            right: tree_map! {
                                key: 6,
                                value: "eta",
                            }
                        }
                    }
                }
            }
        }
    };

    println!("### before splay ###");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "(0(1(2(3(4(5(6)))))))");

    // スプレー操作を行う
    tree.splay(&6);

    println!("### after splay ###");
    println!("{:?}", &tree);
    assert_eq!(&tree.encode(), "(((0)1((2)3((4)5)))6)");
}

#[test]
fn test_splay_not_found() {
    let mut tree: SplayTreeMap<u8, &str> = SplayTreeMap::new();

    tree.root = tree_map! {
        key: 1,
        value: "alpha",
        right: tree_map! {
            key: 2,
            value: "beta",
            right: tree_map! {
                key: 4,
                value: "gamma",
                right: tree_map! {
                    key: 8,
                    value: "delta",
                    right: tree_map! {
                        key: 16,
                        value: "epsilon",
                        right: tree_map! {
                            key: 32,
                            value: "zeta",
                            right: tree_map! {
                                key: 64,
                                value: "eta",
                            }
                        }
                    }
                }
            }
        }
    };

    println!("{:?}", &tree);

    println!("> Splay for 3");
    tree.splay(&3);
    println!("{:?}", &tree);

    println!("> Splay for 50");
    tree.splay(&50);
    println!("{:?}", &tree);

    println!("> Splay for 3");
    tree.splay(&3);
    println!("{:?}", &tree);
}
