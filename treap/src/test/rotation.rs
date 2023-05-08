#![cfg(test)]

use crate::treap::*;

#[test]
fn test_rotate() {
    let mut root = Some(Box::new(TreapNode {
        priority: 0.0,
        value: 4,
        left: Some(Box::new(TreapNode {
            priority: 0.0,
            value: 2,
            left: Some(Box::new(TreapNode {
                priority: 0.0,
                value: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreapNode {
                priority: 0.0,
                value: 3,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreapNode {
            priority: 0.0,
            value: 5,
            left: None,
            right: None,
        })),
    }));

    println!("----- 回転前 -----");
    pretty_print_inner(&root, 0);

    // ## 右回転のテスト
    // 右回転
    root = rotate_right(root);

    println!("----- 右回転 -----");
    pretty_print_inner(&root, 0);

    // さらに右回転
    root = rotate_right(root);

    println!("----- 右回転 -----");
    pretty_print_inner(&root, 0);

    // さらに右回転
    root = rotate_right(root);

    println!("----- 右回転 -----");
    pretty_print_inner(&root, 0);

    // ## 左回転のテスト
    // 左回転
    root = rotate_left(root);

    println!("----- 左回転 -----");
    pretty_print_inner(&root, 0);

    // さらに左回転
    root = rotate_left(root);

    println!("----- 左回転 -----");
    pretty_print_inner(&root, 0);

    // さらに左回転
    root = rotate_left(root);

    println!("----- 左回転 -----");
    pretty_print_inner(&root, 0);
}
