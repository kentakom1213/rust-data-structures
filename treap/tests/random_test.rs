#![cfg(test)]

use rand::prelude::*;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};
use treap::treap::*;

#[test]
fn test_insert_delete_random_u8() {
    let mut tree = Treap::<u8>::new();

    for _ in 0..100_000 {
        // 挿入
        let x = random();
        tree.insert(x);

        // 削除
        let y = random();
        tree.discard(&y);
    }
}

#[test]
fn test_random_search() {
    let mut tree = Treap::<u8>::new();
    let mut set = BTreeSet::<u8>::new();

    for _ in 0..100_000 {
        // 挿入
        let x = random();
        tree.insert(x);
        set.insert(x);

        // 探索
        let y = random();
        let lb_y_actual = tree.lower_bound(&y);
        let lb_y_expected = set.range(y..).next();
        assert_eq!(lb_y_actual, lb_y_expected);

        let rb_y_actual = tree.upper_bound(&y);
        let rb_y_expected = set.range((Excluded(y), Unbounded)).next();
        assert_eq!(rb_y_actual, rb_y_expected);
    }
}
