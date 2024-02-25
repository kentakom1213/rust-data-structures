use aa_tree::{
    node::{delete, insert},
    print_util::pretty_print,
};

#[test]
fn test_insert() {
    let mut tree = None;

    println!("  default ");
    pretty_print(&tree);

    for (i, c) in ('A'..='Z').enumerate() {
        tree = insert(tree, c, i);

        println!("> insert {c}");
        pretty_print(&tree);
    }
}

#[test]
fn test_insert_rev() {
    let mut tree = None;

    println!("  default");
    pretty_print(&tree);

    for (i, c) in ('A'..='Z').rev().enumerate() {
        tree = insert(tree, c, i);

        println!("> insert {c}");
        pretty_print(&tree);
    }
}

#[test]
fn test_delete() {
    let mut tree = None;

    for (i, c) in ('A'..='H').enumerate() {
        tree = insert(tree, c, i);
    }

    println!("  default ");
    pretty_print(&tree);

    for c in 'A'..='J' {
        println!("> delete {c}");

        tree = delete(tree, &c).0;

        pretty_print(&tree);
    }
}

#[test]
fn test_complete() {
    let mut tree = None;

    for i in 1..=7 {
        tree = insert(tree, i, i);
    }

    pretty_print(&tree);

    // 4を削除
    (tree, _) = delete(tree, &2);

    pretty_print(&tree);
}
