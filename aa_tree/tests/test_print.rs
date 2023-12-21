use aa_tree::{
    node::{AATreeNode, AATreeNodeInner},
    print_util::pretty_print,
    tree,
};

#[test]
fn test_tree() {
    let null_tree: AATreeNode<&str, i32> = None;

    pretty_print(&null_tree);

    let mono_tree = tree! {
        key: "root",
        value: 0,
        level: 1,
    };

    pretty_print(&mono_tree);

    /*

       |
       B - D
      /   / \
     A   C   E

    */
    let tree = tree! {
        key: "B",
        value: 2,
        level: 2,
        left: tree! {
            key: "A",
            value: 1,
            level: 1,
        },
        right: tree! {
            key: "D",
            value: 4,
            level: 2,
            left: tree! {
                key: "C",
                value: 3,
                level: 1,
            },
            right: tree! {
                key: "E",
                value: 5,
                level: 1,
            }
        }
    };

    pretty_print(&tree);
}
