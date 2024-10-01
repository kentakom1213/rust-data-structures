use splay_tree_unsafe::{multiset::Multiset, node::pointer::NodeOps};

#[test]
fn test_insert() {
    let mut mset = Multiset::new();
    assert_eq!(format!("{mset:?}"), "{}".to_string());
    assert_eq!(mset.len(), 0);
    assert_eq!(mset.count(&1), 0);

    for i in 0..5 {
        mset.insert(i);
    }

    mset.print_as_tree();
    assert_eq!(format!("{mset:?}"), "{0, 1, 2, 3, 4}".to_string());
    assert_eq!(mset.len(), 5);
    assert_eq!(mset.count(&1), 1);

    for i in 0..5 {
        mset.insert(i);
    }

    mset.print_as_tree();
    assert_eq!(
        format!("{mset:?}"),
        "{0, 0, 1, 1, 2, 2, 3, 3, 4, 4}".to_string()
    );
    assert_eq!(mset.len(), 10);
    assert_eq!(mset.count(&1), 2);

    for i in 0..5 {
        mset.insert(i);
    }

    mset.print_as_tree();
    assert_eq!(
        format!("{mset:?}"),
        "{0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4}".to_string()
    );
    assert_eq!(mset.len(), 15);
    assert_eq!(mset.count(&1), 3);
}

#[test]
fn test_remove() {
    let mut mset = Multiset::new();

    for _ in 0..3 {
        for i in 0..5 {
            mset.insert(i);
        }
    }

    mset.print_as_tree();
    assert_eq!(
        format!("{mset:?}"),
        "{0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4}".to_string()
    );
    assert_eq!(mset.len(), 15);
    assert_eq!(mset.count(&2), 3);

    assert_eq!(mset.remove(&2), true);
    mset.print_as_tree();
    assert_eq!(
        format!("{mset:?}"),
        "{0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 4}".to_string()
    );
    assert_eq!(mset.len(), 14);
    assert_eq!(mset.count(&2), 2);

    assert_eq!(mset.remove(&2), true);
    mset.print_as_tree();
    assert_eq!(
        format!("{mset:?}"),
        "{0, 0, 0, 1, 1, 1, 2, 3, 3, 3, 4, 4, 4}".to_string()
    );
    assert_eq!(mset.len(), 13);
    assert_eq!(mset.count(&2), 1);

    assert_eq!(mset.remove(&2), true);
    mset.print_as_tree();
    assert_eq!(
        format!("{mset:?}"),
        "{0, 0, 0, 1, 1, 1, 3, 3, 3, 4, 4, 4}".to_string()
    );
    assert_eq!(mset.len(), 12);
    assert_eq!(mset.count(&2), 0);

    assert_eq!(mset.remove(&2), false);
    mset.print_as_tree();
    assert_eq!(
        format!("{mset:?}"),
        "{0, 0, 0, 1, 1, 1, 3, 3, 3, 4, 4, 4}".to_string()
    );
    assert_eq!(mset.len(), 12);
    assert_eq!(mset.count(&2), 0);
}

#[test]
fn test_iter() {
    let mut mset = Multiset::new();

    let mut data = vec![9, 0, 1, 3, 90, 43, 30, 22, 43, 20];

    for i in data.iter() {
        mset.insert(*i);
    }

    data.sort();

    for (node, &i) in mset.iter().zip(&data) {
        assert_eq!(*node.key(), i);
    }
}

#[test]
fn test_range() {
    let mut mset = Multiset::new();

    let mut data = vec![5, 8, 1, 1, 6, 9, 7, 0, 7, 1, 2, 4, 6, 3];

    for i in data.iter() {
        mset.insert(*i);
    }

    mset.print_as_tree();

    data.sort();

    let range = mset.range(20..);

    for node in range {
        print!("{}, ", *node.key());
    }
    println!();

    let range = mset.range(1..3);

    for (node, i) in range.zip([1, 1, 1, 2].into_iter()) {
        assert_eq!(*node.key(), i);
    }

    let range = mset.range(3..=6);

    for (node, i) in range.zip([3, 4, 5, 6, 6].into_iter()) {
        assert_eq!(*node.key(), i);
    }

    let range = mset.range(7..);

    for (node, i) in range.zip([7, 7, 8, 9].into_iter()) {
        assert_eq!(*node.key(), i);
    }

    let range = mset.range(..3);

    for (node, i) in range.zip([0, 1, 1, 1, 2].into_iter()) {
        assert_eq!(*node.key(), i);
    }

    let range = mset.range(..);

    for (node, i) in range.zip([0, 1, 1, 1, 2, 3, 4, 5, 6, 6, 7, 7, 8, 9].into_iter()) {
        assert_eq!(*node.key(), i);
    }

    let range = mset.range(..=3);

    for (node, i) in range.rev().zip([3, 2, 1, 1, 1, 0].into_iter()) {
        assert_eq!(*node.key(), i);
    }
}
