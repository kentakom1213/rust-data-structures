use splay_tree_traversable::splay_tree::SplayTree;

#[test]
fn test_node() {
    let mut tree: SplayTree<usize, usize> = SplayTree::new();

    tree.pretty_print();

    tree.insert(0, 20);

    tree.pretty_print();

    let old = tree.insert(0, 100);

    assert_eq!(old, Some(20));

    tree.pretty_print();
}
