use splay_tree::{
    node::{find::find, insert::insert},
    utils::print::print_as_tree,
};

#[test]
fn test_node() {
    let mut root = None;

    for i in 0..20 {
        (root, _, _) = insert(root, i, format!("{i}"));
    }

    (root, _) = find(root, &20);

    print_as_tree(&root);
}
