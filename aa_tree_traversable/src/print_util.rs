//! 木を整形して表示するための関数

use crate::node::AATreeNode;
use std::fmt::Debug;

const GREEN: &str = "\x1b[92m";
const BLUE: &str = "\x1b[94m";
const END: &str = "\x1b[0m";

/// B木（2-3木）として出力する
pub fn print_as_btree<K, V>(root: &AATreeNode<K, V>)
where
    K: Ord + Debug,
    V: Debug,
{
    println!("{GREEN}┌─ BTree ───────────────{END}");
    fmt_inner_btree(root, root.as_ref().map_or(0, |node| node.borrow().level));
    println!("{GREEN}└───────────────────────{END}");
}

/// print recursive
fn fmt_inner_btree<K, V>(node: &AATreeNode<K, V>, depth: usize)
where
    K: Ord + Debug,
    V: Debug,
{
    if let Some(node) = node.as_ref() {
        fmt_inner_btree(&node.borrow().left, depth);
        println!(
            "{GREEN}│{END}{}[{:?}: {:?}]",
            "    ".repeat(depth - node.borrow().level),
            node.borrow().key,
            node.borrow().value
        );
        fmt_inner_btree(&node.borrow().right, depth);
    }
}

/// 2分木として出力する
pub fn print_as_binary_tree<K, V>(root: &AATreeNode<K, V>)
where
    K: Ord + Debug,
    V: Debug,
{
    println!("{BLUE}┌─ BinaryTree ──────────{END}");
    fmt_inner_binary_tree(root, 0);
    println!("{BLUE}└───────────────────────{END}");
}

/// print recursive
fn fmt_inner_binary_tree<K, V>(node: &AATreeNode<K, V>, depth: usize)
where
    K: Ord + Debug,
    V: Debug,
{
    if let Some(node) = node.as_ref() {
        fmt_inner_binary_tree(&node.borrow().left, depth + 1);
        println!(
            "{BLUE}│{END}{}[{:?}: {:?}]",
            "    ".repeat(depth),
            node.borrow().key,
            node.borrow().value
        );
        fmt_inner_binary_tree(&node.borrow().right, depth + 1);
    }
}
