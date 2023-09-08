#![cfg(test)]

use splay_tree::{splay_tree::*, tree};

#[test]
fn test_from_path() {
    let mut tree: SplayTree<u8, i32> = SplayTree::new();

    tree.root = tree! {
        key: 0,
        value: 0,
        right: tree! {
            key: 1,
            value: 1,
            right: tree! {
                key: 2,
                value: 2,
                right: tree! {
                    key: 3,
                    value: 3,
                    right: tree! {
                        key: 4,
                        value: 4,
                        right: tree! {
                            key: 5,
                            value: 5,
                        }
                    }
                }
            }
        }
    };

    println!("### before splay ###");
    println!("{:?}", &tree);

    assert_eq!(tree.search(&5), Some(&5));
    assert_eq!(tree.search(&6), None);

    // スプレー操作を行う
    tree.splay(&0);

    println!("### after splay ###");
    println!("{:?}", &tree);
}
