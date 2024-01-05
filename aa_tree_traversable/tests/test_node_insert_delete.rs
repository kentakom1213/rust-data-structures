use aa_tree_traversable::{
    node::insert,
    print_util::{print_as_binary_tree, print_as_btree},
};

#[test]
fn test_insert() {
    let mut tree = None;

    println!("  default ");
    print_as_btree(&tree);

    for (i, c) in ('A'..='Z').enumerate() {
        tree = insert(tree, c, i);

        println!("> insert {c}");
        print_as_btree(&tree);
        print_as_binary_tree(&tree);
    }
}

#[test]
fn test_insert_rev() {
    let mut tree = None;

    println!("  default ");
    print_as_btree(&tree);

    for (i, c) in ('A'..='Z').rev().enumerate() {
        tree = insert(tree, c, i);

        println!("> insert {c}");
        print_as_btree(&tree);
        print_as_binary_tree(&tree);
    }
}

#[test]
fn test_insert_shuffle() {
    let mut tree = None;

    println!("  default");
    print_as_btree(&tree);

    let alphas = vec![
        'M', 'N', 'F', 'S', 'U', 'Q', 'H', 'T', 'D', 'E', 'W', 'R', 'P', 'Y', 'V', 'L', 'O', 'G',
        'Z', 'I', 'X', 'K', 'C', 'B', 'A', 'J',
    ];

    for (i, &c) in alphas.iter().enumerate() {
        tree = insert(tree, c, i);

        println!("> insert {c}");
        print_as_btree(&tree);
        print_as_binary_tree(&tree);
    }
}

#[test]
fn test_int_increment() {
    let mut tree = None;

    for i in 1..=15 {
        tree = insert(tree, i, i);
        print_as_binary_tree(&tree);
    }
}

// #[test]
// fn test_delete() {
//     let mut tree = None;

//     for (i, c) in ('A'..='H').enumerate() {
//         tree = insert(tree, c, i);
//     }

//     println!("  default ");
//     print_as_btree(&tree);

//     for c in 'A'..='J' {
//         println!("> delete {c}");

//         tree = delete(tree, &c).0;

//         print_as_btree(&tree);
//     }
// }


/*

     ┌─ [  ]
 ┌─ [  ]
 │   └─ [  ]
[  ]
 │   ┌─ [  ]
 └─ [  ]
     └─ [  ]


*/
