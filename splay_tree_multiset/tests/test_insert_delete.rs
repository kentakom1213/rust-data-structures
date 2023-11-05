use rand::random;
use splay_tree_multiset::multiset::*;

#[test]
fn test_insert() {
    let mut multiset = SplayTreeMultiSet::new();

    assert_eq!(multiset.len(), 0);

    println!("--- insert 1 ---");
    multiset.insert(1);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 1);

    println!("--- insert 1 ---");
    multiset.insert(1);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 2);

    println!("--- insert 1 ---");
    multiset.insert(1);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 3);

    println!("--- insert 1 ---");
    multiset.insert(1);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 4);

    println!("--- insert 0 ---");
    multiset.insert(0);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 5);

    println!("--- insert 2 ---");
    multiset.insert(2);
    println!("{:?}", &multiset);

    assert_eq!(multiset.len(), 6);
}

#[test]
fn test_insert_sorted_greater() {
    let multiset: SplayTreeMultiSet<usize> = (0..20).collect();

    println!("{:#?}", multiset);
}

#[test]
fn test_insert_sorted_less() {
    let multiset: SplayTreeMultiSet<usize> = (0..20).rev().collect();

    println!("{:#?}", multiset);
}

#[test]
fn test_insert_random() {
    let mut multiset = SplayTreeMultiSet::<u8>::new();

    for _ in 0..50 {
        let x = random();
        multiset.insert(x);

        println!("--- insert {} ---", x);
        println!("{:#?}", multiset);
    }
}

/*
ERROR
---- test_lb_ub_random stdout ----
[49, 117, 24, 188, 64, 9, 189, 64, 105, 3]
[3, 9, 24, 49, 64, 64, 105, 117, 188, 189]
3
  9
        24
          49
      64
        117
    64
      105
        188
          189
*/
