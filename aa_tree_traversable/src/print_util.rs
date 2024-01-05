//! 木を整形して表示するための関数

use crate::node::AATreeNode;
use std::fmt::Debug;

const GREEN: &str = "\x1b[92m";
const BLUE: &str = "\x1b[94m";
const END: &str = "\x1b[0m";
const LEFT: &str = " ┌──";
const MID: &str = " │  ";
const RIGHT: &str = " └──";
const NULL: &str = "";
const BLANK: &str = "    ";

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
    fmt_inner_binary_tree(root, &mut vec![], NULL);
    println!("{BLUE}└───────────────────────{END}");
}

/// print recursive
fn fmt_inner_binary_tree<K, V>(node: &AATreeNode<K, V>, fill: &mut Vec<&'static str>, last: &'static str)
    where
        K: Ord + Debug,
        V: Debug,
{
    if let Some(node) = node.as_ref() {
        // 表示の調整
        let mut tmp = None;
        if fill.last().is_some_and(|x| x == &last) {
            tmp = fill.pop();
            fill.push(BLANK);
        } else if fill.last().is_some_and(|x| x != &NULL && x != &BLANK) {
            tmp = fill.pop();
            fill.push(MID);
        }
        fill.push(last);
        // 左の子
        fmt_inner_binary_tree(&node.borrow().left, fill, LEFT);
        // 自分を出力
        println!(
            "{BLUE}│{END}{}[{:?}: {:?}]",
            fill.iter().fold(String::new(), |s, x| s + x),
            node.borrow().key,
            node.borrow().value
        );
        // 右の子
        fmt_inner_binary_tree(&node.borrow().right, fill, RIGHT);
        fill.pop();
        // 戻す
        if let Some(tmp) = tmp {
            fill.pop();
            fill.push(tmp);
        }
    }
}
