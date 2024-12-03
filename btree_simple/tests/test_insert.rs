use btree_simple::{insert_multi, print_as_tree, BTreeNode, NodePtr, NodeUtil};

/// 空きのあるノードに挿入
#[test]
fn test_insert_with_vacent() {
    let mut tree: Option<NodePtr<2, i32, &str>> = Some(BTreeNode::alloc_leaf());

    // 空きがある
    assert_eq!(tree.as_ref().unwrap().is_full(), false);

    println!("> before insertion");
    print_as_tree(&tree);

    let items = vec![
        (20, "0020"),
        (50, "0050"),
        (10, "0010"),
        (30, "0030"),
        (20, "0020(2)"),
        (15, "0015"),
        (13, "0013"),
        (24, "0024"),
        (39, "0039"),
        (100, "0100"),
        (400, "0400"),
        (23, "0023"),
        (22, "0022"),
        (18, "0018"),
    ];

    for (k, v) in items {
        tree = insert_multi(tree, k, v);

        println!("> insert {k}");
        print_as_tree(&tree);
    }
}
