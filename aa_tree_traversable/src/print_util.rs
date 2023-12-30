//! 木を整形して表示するための関数

use crate::node::AATreeNodeChild;
use std::fmt::Debug;

const GREEN: &str = "\x1b[92m";
const END: &str = "\x1b[0m";

pub fn pretty_print<K, V>(root: &AATreeNodeChild<K, V>)
where
    K: Ord + Debug,
    V: Debug,
{
    println!("{GREEN}┌─ Tree ───────────────{END}");
    fmt_inner(root, root.as_ref().map_or(0, |node| node.borrow().level));
    println!("{GREEN}└──────────────────────{END}");
}

/// print recursive
fn fmt_inner<K, V>(node: &AATreeNodeChild<K, V>, depth: usize)
where
    K: Ord + Debug,
    V: Debug,
{
    if let Some(node) = node.as_ref() {
        fmt_inner(&node.borrow().left, depth);
        println!(
            "{GREEN}│{END}{}({:?}, {:?})",
            "    ".repeat(depth - node.borrow().level),
            node.borrow().key,
            node.borrow().value
        );
        fmt_inner(&node.borrow().right, depth);
    }
}
