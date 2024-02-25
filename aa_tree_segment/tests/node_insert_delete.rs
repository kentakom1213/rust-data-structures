use std::{collections::VecDeque, ops::Bound::*};

use aa_tree_segment::{
    alg::monoids::{Add, Str},
    node::*,
    print_util::print_as_binary_tree,
};
use rand::{
    distributions::{Alphanumeric, DistString},
    prelude::*,
};

#[test]
#[rustfmt::skip]
fn test_insert() {
    let mut seg: Option<Box<NodeInner<i32, Add>>> = None;

    assert_eq!(get_range(&seg, Included(&10), Excluded(&0), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&10), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&1), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&2), Excluded(&8), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&3), Excluded(&6), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&4), Excluded(&9), Unbounded, Unbounded), 0);

    // [(2: 5)]
    (seg, _) = insert(seg, 2, 5);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, Included(&10), Excluded(&0), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&10), Unbounded, Unbounded), 5);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&1), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&2), Excluded(&8), Unbounded, Unbounded), 5);
    assert_eq!(get_range(&seg, Included(&3), Excluded(&6), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&4), Excluded(&9), Unbounded, Unbounded), 0);

    // [(2: 5), (5: 8)]
    (seg, _) = insert(seg, 5, 8);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, Included(&10), Excluded(&0), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&10), Unbounded, Unbounded), 13);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&1), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&2), Excluded(&8), Unbounded, Unbounded), 13);
    assert_eq!(get_range(&seg, Included(&3), Excluded(&6), Unbounded, Unbounded), 8);
    assert_eq!(get_range(&seg, Included(&4), Excluded(&9), Unbounded, Unbounded), 8);

    // [(2: 5), (3: 3), (5: 8)]
    (seg, _) = insert(seg, 3, 3);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, Included(&10), Excluded(&0), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&10), Unbounded, Unbounded), 16);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&1), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&2), Excluded(&8), Unbounded, Unbounded), 16);
    assert_eq!(get_range(&seg, Included(&3), Excluded(&6), Unbounded, Unbounded), 11);
    assert_eq!(get_range(&seg, Included(&4), Excluded(&9), Unbounded, Unbounded), 8);

    // [(2: 5), (3: 3), (5: 8), (8: 1)]
    (seg, _) = insert(seg, 8, 1);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, Included(&10), Excluded(&0), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&10), Unbounded, Unbounded), 17);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&1), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&2), Excluded(&8), Unbounded, Unbounded), 16);
    assert_eq!(get_range(&seg, Included(&3), Excluded(&6), Unbounded, Unbounded), 11);
    assert_eq!(get_range(&seg, Included(&4), Excluded(&9), Unbounded, Unbounded), 9);

    // [(2: 5), (3: 3), (4: 6), (5: 8), (8: 1)]
    (seg, _) = insert(seg, 4, 6);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, Included(&10), Excluded(&0), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&10), Unbounded, Unbounded), 23);
    assert_eq!(get_range(&seg, Included(&0), Excluded(&1), Unbounded, Unbounded), 0);
    assert_eq!(get_range(&seg, Included(&2), Excluded(&8), Unbounded, Unbounded), 22);
    assert_eq!(get_range(&seg, Included(&3), Excluded(&6), Unbounded, Unbounded), 17);
    assert_eq!(get_range(&seg, Included(&4), Excluded(&9), Unbounded, Unbounded), 15);
}

#[test]
fn test_noncommutative() {
    let mut seg: Node<usize, Str> = None;

    for (i, c) in ('A'..='G').enumerate() {
        (seg, _) = insert(seg, i, c.to_string());
        print_as_binary_tree(&seg);
    }

    assert_eq!(
        &get_range(&seg, Included(&5), Excluded(&6), Unbounded, Unbounded),
        "F"
    );
    assert_eq!(
        &get_range(&seg, Included(&4), Excluded(&20), Unbounded, Unbounded),
        "EFG"
    );
    assert_eq!(
        &get_range(&seg, Included(&0), Excluded(&7), Unbounded, Unbounded),
        "ABCDEFG"
    );
    assert_eq!(
        &get_range(&seg, Included(&1), Excluded(&5), Unbounded, Unbounded),
        "BCDE"
    );
    assert_eq!(
        &get_range(&seg, Included(&0), Excluded(&1), Unbounded, Unbounded),
        "A"
    );
    assert_eq!(
        &get_range(&seg, Included(&6), Excluded(&7), Unbounded, Unbounded),
        "G"
    );
}

#[test]
fn test_delete_mini() {
    let mut seg: Node<_, Add> = None;

    for i in 1..=3 {
        (seg, _) = insert(seg, i, i);
    }

    print_as_binary_tree(&seg);

    (seg, _) = delete(seg, &2);

    print_as_binary_tree(&seg);
}

#[test]
fn test_insert_delete() {
    let mut seg: Node<_, Add> = None;

    for i in 1..=7 {
        (seg, _) = insert(seg, i, i);
    }

    print_as_binary_tree(&seg);

    println!("> delete 4");
    (seg, _) = delete(seg, &4);
    print_as_binary_tree(&seg);

    println!("> delete 3");
    (seg, _) = delete(seg, &3);
    print_as_binary_tree(&seg);

    println!("> delete 1");
    (seg, _) = delete(seg, &1);
    print_as_binary_tree(&seg);

    println!("> delete 1");
    (seg, _) = delete(seg, &1);
    print_as_binary_tree(&seg);

    println!("> delete 5");
    (seg, _) = delete(seg, &5);
    print_as_binary_tree(&seg);

    println!("> delete 6");
    (seg, _) = delete(seg, &6);
    print_as_binary_tree(&seg);

    println!("> delete 7");
    (seg, _) = delete(seg, &7);
    print_as_binary_tree(&seg);

    println!("> delete 2");
    (seg, _) = delete(seg, &2);
    print_as_binary_tree(&seg);

    println!("> delete 10");
    (seg, _) = delete(seg, &10);
    print_as_binary_tree(&seg);
}

#[test]
fn test_random_insert() {
    const ITER: usize = 1000;
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

            assert_eq!(
                arr[l..r].iter().sum::<isize>(),
                get_range(&seg, Included(&l), Excluded(&r), Unbounded, Unbounded)
            );
        }
    }
}

#[test]
fn random_insert_delete() {
    const ITER: usize = 1000;
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
        let idx_insert = rng.gen_range(0..SIZE);
        let idx_delete = rng.gen_range(0..SIZE);
        let new_val: isize = rng.gen_range(-1_000_000_000..1_000_000_000);

        // 配列の更新
        arr[idx_insert] = new_val;
        arr[idx_delete] = 0;

        // セグ木の更新
        (seg, _) = insert(seg, idx_insert, new_val);
        (seg, _) = delete(seg, &idx_delete);

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

            assert_eq!(
                arr[l..r].iter().sum::<isize>(),
                get_range(&seg, Included(&l), Excluded(&r), Unbounded, Unbounded)
            );
        }
    }
}

#[test]
fn random_delete() {
    const ITER: usize = 400;
    const QUERY: usize = 400;

    let mut rng = rand::thread_rng();

    // 配列
    let mut arr: Vec<(isize, isize)> = vec![];

    // セグ木
    let mut seg: Node<isize, Add> = None;

    // ランダムな値を追加
    for _ in 0..ITER {
        let key = rng.gen();
        let val = rng.gen_range(-1_000_000_000..1_000_000_000);

        let idx_insert = arr.partition_point(|&(k, _)| k < key);

        // 同じキーのときの処理
        if idx_insert < arr.len() && arr[idx_insert].0 == key {
            continue;
        }

        // 配列に追加
        arr.insert(idx_insert, (key, val));

        // セグ木に追加
        (seg, _) = insert(seg, key, val);
    }

    // println!("{:?}", arr);
    // print_as_binary_tree(&seg);

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx_delete = rng.gen_range(0..arr.len());
        let (key, arr_delete_val) = arr.remove(idx_delete);

        // セグ木の更新
        let (new_seg, seg_delete_val) = delete(seg, &key);
        seg = new_seg;

        // 削除した値は等しいか
        assert_eq!(arr_delete_val, seg_delete_val.unwrap().1);

        // 表示
        // println!("{:?}", arr);
        // print_as_binary_tree(&seg);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let (mut l, mut r) = (rng.gen(), rng.gen());
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(
                arr.iter()
                    .filter(|&&(k, _)| l <= k && k < r)
                    .map(|&(_, v)| v)
                    .sum::<isize>(),
                get_range(&seg, Included(&l), Excluded(&r), Unbounded, Unbounded)
            );
        }
    }
}

#[test]
fn random_delete_str() {
    const ITER: usize = 200;
    const QUERY: usize = 200;
    const SIZE: usize = 10;

    let mut rng = rand::thread_rng();

    // 配列
    let mut arr: Vec<(String, isize)> = vec![];

    // セグ木
    let mut seg: Node<String, Add> = None;

    // ランダムな値を追加
    for _ in 0..ITER {
        let key = Alphanumeric.sample_string(&mut rng, SIZE);
        let val = rng.gen_range(-1_000_000_000..1_000_000_000);

        let idx_insert = arr.partition_point(|(k, _)| k < &key);

        // 同じキーのときの処理
        if idx_insert < arr.len() && arr[idx_insert].0 == key {
            continue;
        }

        // 配列に追加
        arr.insert(idx_insert, (key.clone(), val));

        // セグ木に追加
        (seg, _) = insert(seg, key, val);
    }

    println!("{:?}", arr);
    print_as_binary_tree(&seg);

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx_delete = rng.gen_range(0..arr.len());
        let (key, arr_delete_val) = arr.remove(idx_delete);

        // セグ木の更新
        let (new_seg, seg_delete_val) = delete(seg, &key);
        seg = new_seg;

        // 削除した値は等しいか
        assert_eq!(arr_delete_val, seg_delete_val.unwrap().1);

        // 表示
        // println!("{:?}", arr);
        // print_as_binary_tree(&seg);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let mut l = Alphanumeric.sample_string(&mut rng, SIZE);
            let mut r = Alphanumeric.sample_string(&mut rng, SIZE);
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(
                arr.iter()
                    .filter(|(k, _)| &l <= k && k < &r)
                    .map(|&(_, v)| v)
                    .sum::<isize>(),
                get_range(&seg, Included(&l), Excluded(&r), Unbounded, Unbounded)
            );
        }
    }
}

#[test]
fn random_deq_delete_str() {
    const ITER: usize = 200;
    const QUERY: usize = 200;
    const SIZE: usize = 10;

    let mut rng = rand::thread_rng();

    // 配列
    let mut arr: VecDeque<(String, isize)> = VecDeque::new();

    // セグ木
    let mut seg: Node<String, Add> = None;

    // ランダムな値を追加
    for _ in 0..ITER {
        let key = Alphanumeric.sample_string(&mut rng, SIZE);
        let val = rng.gen_range(-1_000_000_000..1_000_000_000);

        let idx_insert = arr.partition_point(|(k, _)| k < &key);

        // 同じキーのときの処理
        if idx_insert < arr.len() && arr[idx_insert].0 == key {
            continue;
        }

        // 配列に追加
        arr.insert(idx_insert, (key.clone(), val));
    }

    for (key, val) in arr.iter() {
        (seg, _) = insert(seg, key.clone(), *val);
    }

    println!("{:?}", arr);
    print_as_binary_tree(&seg);

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let (key, arr_delete_val) = arr.pop_front().unwrap();

        // セグ木の更新
        let (new_seg, seg_delete_val) = delete(seg, &key);
        seg = new_seg;

        // 削除した値は等しいか
        assert_eq!(arr_delete_val, seg_delete_val.unwrap().1);

        // 表示
        // println!("{:?}", arr);
        print_as_binary_tree(&seg);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let mut l = Alphanumeric.sample_string(&mut rng, SIZE);
            let mut r = Alphanumeric.sample_string(&mut rng, SIZE);
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(
                arr.iter()
                    .filter(|(k, _)| &l <= k && k < &r)
                    .map(|&(_, v)| v)
                    .sum::<isize>(),
                get_range(&seg, Included(&l), Excluded(&r), Unbounded, Unbounded)
            );
        }
    }
}
