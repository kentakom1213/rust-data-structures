use std::ops::Bound::*;

use aa_tree_segment::{
    alg::monoids::{Add, Str},
    node::{get_range, insert, Node},
    print_util::print_as_binary_tree,
};
use rand::Rng;

#[test]
#[rustfmt::skip]
fn test_noncommutative_bound() {
    let mut seg: Node<usize, Str> = None;

    for (i, c) in ('A'..='G').enumerate() {
        (seg, _) = insert(seg, i, c.to_string());
    }

    print_as_binary_tree(&seg);

    // 0123456
    // ABCDEFG

    // 5 -> 6
    assert_eq!(&get_range(&seg, Included(&5), Included(&6), Unbounded, Unbounded), "FG");
    assert_eq!(&get_range(&seg, Included(&5), Excluded(&6), Unbounded, Unbounded), "F");
    assert_eq!(&get_range(&seg, Included(&5), Unbounded, Unbounded, Unbounded), "FG");
    assert_eq!(&get_range(&seg, Excluded(&5), Included(&6), Unbounded, Unbounded), "G");
    assert_eq!(&get_range(&seg, Excluded(&5), Excluded(&6), Unbounded, Unbounded), "");
    assert_eq!(&get_range(&seg, Excluded(&5), Unbounded, Unbounded, Unbounded), "G");
    assert_eq!(&get_range(&seg, Unbounded, Included(&6), Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Excluded(&6), Unbounded, Unbounded), "ABCDEF");
    assert_eq!(&get_range(&seg, Unbounded, Unbounded, Unbounded, Unbounded), "ABCDEFG");

    // 4 -> 9
    assert_eq!(&get_range(&seg, Included(&4), Included(&9), Unbounded, Unbounded), "EFG");
    assert_eq!(&get_range(&seg, Included(&4), Excluded(&9), Unbounded, Unbounded), "EFG");
    assert_eq!(&get_range(&seg, Included(&4), Unbounded, Unbounded, Unbounded), "EFG");
    assert_eq!(&get_range(&seg, Excluded(&4), Included(&9), Unbounded, Unbounded), "FG");
    assert_eq!(&get_range(&seg, Excluded(&4), Excluded(&9), Unbounded, Unbounded), "FG");
    assert_eq!(&get_range(&seg, Excluded(&4), Unbounded, Unbounded, Unbounded), "FG");
    assert_eq!(&get_range(&seg, Unbounded, Included(&9), Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Excluded(&9), Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Unbounded, Unbounded, Unbounded), "ABCDEFG");

    // 0 -> 7
    assert_eq!(&get_range(&seg, Included(&0), Included(&7), Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Included(&0), Excluded(&7), Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Included(&0), Unbounded, Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Excluded(&0), Included(&7), Unbounded, Unbounded), "BCDEFG");
    assert_eq!(&get_range(&seg, Excluded(&0), Excluded(&7), Unbounded, Unbounded), "BCDEFG");
    assert_eq!(&get_range(&seg, Excluded(&0), Unbounded, Unbounded, Unbounded), "BCDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Included(&7), Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Excluded(&7), Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Unbounded, Unbounded, Unbounded), "ABCDEFG");

    // 1 -> 5
    assert_eq!(&get_range(&seg, Included(&1), Included(&5), Unbounded, Unbounded), "BCDEF");
    assert_eq!(&get_range(&seg, Included(&1), Excluded(&5), Unbounded, Unbounded), "BCDE");
    assert_eq!(&get_range(&seg, Included(&1), Unbounded, Unbounded, Unbounded), "BCDEFG");
    assert_eq!(&get_range(&seg, Excluded(&1), Included(&5), Unbounded, Unbounded), "CDEF");
    assert_eq!(&get_range(&seg, Excluded(&1), Excluded(&5), Unbounded, Unbounded), "CDE");
    assert_eq!(&get_range(&seg, Excluded(&1), Unbounded, Unbounded, Unbounded), "CDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Included(&5), Unbounded, Unbounded), "ABCDEF");
    assert_eq!(&get_range(&seg, Unbounded, Excluded(&5), Unbounded, Unbounded), "ABCDE");
    assert_eq!(&get_range(&seg, Unbounded, Unbounded, Unbounded, Unbounded), "ABCDEFG");

    // 0 -> 1
    assert_eq!(&get_range(&seg, Included(&0), Included(&1), Unbounded, Unbounded), "AB");
    assert_eq!(&get_range(&seg, Included(&0), Excluded(&1), Unbounded, Unbounded), "A");
    assert_eq!(&get_range(&seg, Included(&0), Unbounded, Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Excluded(&0), Included(&1), Unbounded, Unbounded), "B");
    assert_eq!(&get_range(&seg, Excluded(&0), Excluded(&1), Unbounded, Unbounded), "");
    assert_eq!(&get_range(&seg, Excluded(&0), Unbounded, Unbounded, Unbounded), "BCDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Included(&1), Unbounded, Unbounded), "AB");
    assert_eq!(&get_range(&seg, Unbounded, Excluded(&1), Unbounded, Unbounded), "A");
    assert_eq!(&get_range(&seg, Unbounded, Unbounded, Unbounded, Unbounded), "ABCDEFG");

    // 6 -> 7
    assert_eq!(&get_range(&seg, Included(&6), Included(&7), Unbounded, Unbounded), "G");
    assert_eq!(&get_range(&seg, Included(&6), Excluded(&7), Unbounded, Unbounded), "G");
    assert_eq!(&get_range(&seg, Included(&6), Unbounded, Unbounded, Unbounded), "G");
    assert_eq!(&get_range(&seg, Excluded(&6), Included(&7), Unbounded, Unbounded), "");
    assert_eq!(&get_range(&seg, Excluded(&6), Excluded(&7), Unbounded, Unbounded), "");
    assert_eq!(&get_range(&seg, Excluded(&6), Unbounded, Unbounded, Unbounded), "");
    assert_eq!(&get_range(&seg, Unbounded, Included(&7), Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Excluded(&7), Unbounded, Unbounded), "ABCDEFG");
    assert_eq!(&get_range(&seg, Unbounded, Unbounded, Unbounded, Unbounded), "ABCDEFG");
}

#[test]
fn test_random_insert() {
    const ITER: usize = 500;
    const QUERY: usize = 400;
    const SIZE: usize = 400;

    let mut rng = rand::thread_rng();

    // 配列
    let mut arr: [isize; SIZE] = [0; SIZE];

    // セグ木
    let mut seg: Node<usize, Add> = None;

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx = rng.gen_range(0..SIZE);
        let new_val: isize = rng.gen_range(-1_000_000_000..1_000_000_000);

        // 配列の更新
        arr[idx] = new_val;

        // セグ木の更新
        (seg, _) = insert(seg, idx, new_val);

        // 表示
        // println!("{:?}", arr);
        // print_as_binary_tree(&seg);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let (mut l, mut r) = (rng.gen_range(0..SIZE), rng.gen_range(0..SIZE));
            if l > r {
                (l, r) = (r, l);
            }

            let exclude_l = (l + 1).min(SIZE);
            let exclude_r = exclude_l.max(r);

            assert_eq!(
                arr[l..=r].iter().sum::<isize>(),
                get_range(&seg, Included(&l), Included(&r), Unbounded, Unbounded)
            );
            assert_eq!(
                arr[l..r].iter().sum::<isize>(),
                get_range(&seg, Included(&l), Excluded(&r), Unbounded, Unbounded)
            );
            assert_eq!(
                arr[exclude_l..=r].iter().sum::<isize>(),
                get_range(&seg, Excluded(&l), Included(&r), Unbounded, Unbounded)
            );
            assert_eq!(
                arr[exclude_l..exclude_r].iter().sum::<isize>(),
                get_range(&seg, Excluded(&l), Excluded(&r), Unbounded, Unbounded)
            );
        }
    }
}
