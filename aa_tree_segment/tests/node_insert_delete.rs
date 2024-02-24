use aa_tree_segment::{
    alg::{monoids::Add, Monoid},
    node::*,
    print_util::{print_as_binary_tree, print_as_btree},
};

#[test]
fn test_insert() {
    let mut seg: Option<Box<NodeInner<i32, Add>>> = None;

    assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
    assert_eq!(get_range(&seg, &0, &10, &0, &10), 0);
    assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
    assert_eq!(get_range(&seg, &2, &8, &0, &10), 0);
    assert_eq!(get_range(&seg, &3, &6, &0, &10), 0);
    assert_eq!(get_range(&seg, &4, &9, &0, &10), 0);

    // [(2: 5)]
    seg = insert(seg, 2, 5);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
    assert_eq!(get_range(&seg, &0, &10, &0, &10), 5);
    assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
    assert_eq!(get_range(&seg, &2, &8, &0, &10), 5);
    assert_eq!(get_range(&seg, &3, &6, &0, &10), 0);
    assert_eq!(get_range(&seg, &4, &9, &0, &10), 0);

    // [(2: 5), (5: 8)]
    seg = insert(seg, 5, 8);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
    assert_eq!(get_range(&seg, &0, &10, &0, &10), 13);
    assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
    assert_eq!(get_range(&seg, &2, &8, &0, &10), 13);
    assert_eq!(get_range(&seg, &3, &6, &0, &10), 8);
    assert_eq!(get_range(&seg, &4, &9, &0, &10), 8);

    // [(2: 5), (3: 3), (5: 8)]
    seg = insert(seg, 3, 3);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
    assert_eq!(get_range(&seg, &0, &10, &0, &10), 16);
    assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
    assert_eq!(get_range(&seg, &2, &8, &0, &10), 16);
    assert_eq!(get_range(&seg, &3, &6, &0, &10), 11);
    assert_eq!(get_range(&seg, &4, &9, &0, &10), 8);

    // [(2: 5), (3: 3), (5: 8), (8: 1)]
    seg = insert(seg, 8, 1);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
    assert_eq!(get_range(&seg, &0, &10, &0, &10), 17);
    assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
    assert_eq!(get_range(&seg, &2, &8, &0, &10), 16);
    assert_eq!(get_range(&seg, &3, &6, &0, &10), 11);
    assert_eq!(get_range(&seg, &4, &9, &0, &10), 9);

    // [(2: 5), (3: 3), (4: 6), (5: 8), (8: 1)]
    seg = insert(seg, 4, 6);
    print_as_binary_tree(&seg);

    assert_eq!(get_range(&seg, &10, &0, &0, &10), 0);
    assert_eq!(get_range(&seg, &0, &10, &0, &10), 23);
    assert_eq!(get_range(&seg, &0, &1, &0, &10), 0);
    assert_eq!(get_range(&seg, &2, &8, &0, &10), 22);
    assert_eq!(get_range(&seg, &3, &6, &0, &10), 17);
    assert_eq!(get_range(&seg, &4, &9, &0, &10), 15);
}

/// 文字列
struct Str;
impl Monoid for Str {
    type Val = String;
    const E: Self::Val = String::new();
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left.to_string() + right
    }
}

#[test]
fn test_noncommutative() {
    let mut seg: Node<usize, Str> = None;

    for (i, c) in ('A'..='G').enumerate() {
        seg = insert(seg, i, c.to_string());
        print_as_binary_tree(&seg);
    }

    assert_eq!(&get_range(&seg, &5, &6, &0, &7), "F");
    assert_eq!(&get_range(&seg, &4, &20, &0, &100), "EFG");
    assert_eq!(&get_range(&seg, &0, &7, &0, &9), "ABCDEFG");
    assert_eq!(&get_range(&seg, &1, &5, &0, &9), "BCDE");
    assert_eq!(&get_range(&seg, &0, &1, &0, &9), "A");
    assert_eq!(&get_range(&seg, &6, &7, &0, &9), "G");
}

#[test]
fn test_delete_mini() {
    let mut seg: Node<_, Add> = None;

    for i in 1..=3 {
        seg = insert(seg, i, i);
    }

    print_as_binary_tree(&seg);

    (seg, _) = delete(seg, &2);

    print_as_binary_tree(&seg);
}

#[test]
fn test_insert_delete() {
    let mut seg: Node<_, Add> = None;

    for i in 1..=7 {
        seg = insert(seg, i, i);
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
