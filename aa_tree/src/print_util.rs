//! 木を整形して表示するための関数

use crate::node::AATreeNode;
use std::fmt::Debug;

pub fn pretty_print<K, V>(root: &AATreeNode<K, V>)
where
    K: Ord + Debug,
    V: Debug,
{
    println!("┌─ Tree ───────────────");
    fmt_inner(root, root.as_ref().map_or(0, |node| node.level));
    println!("└──────────────────────");
}

/// print recursive
fn fmt_inner<K, V>(node: &AATreeNode<K, V>, depth: usize)
where
    K: Ord + Debug,
    V: Debug,
{
    if let Some(node) = node.as_ref() {
        fmt_inner(&node.left, depth);
        println!(
            "│{}({:?}, {:?})",
            "    ".repeat(depth - node.level),
            node.key,
            node.value
        );
        fmt_inner(&node.right, depth);
    }
}
