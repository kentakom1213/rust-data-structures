//! 木を整形して表示するための関数

use crate::{alg::Monoid, dynamic_segment_tree::DynamicSegmentTree, node::Node};
use std::fmt::Debug;

const GREEN: &str = "\x1b[92m";
const BLUE: &str = "\x1b[94m";
const END: &str = "\x1b[0m";
const LEFT: &str = " ┌──";
const MID: &str = " │  ";
const RIGHT: &str = " └──";
const NULL: &str = "";
const BLANK: &str = "    ";

impl<K: Ord + Debug, M: Monoid> DynamicSegmentTree<K, M> {
    /// 2分木として出力する
    pub fn print_as_binary_tree(&self) {
        println!("{BLUE}┌─ BinaryTree ──────────────────────{END}");
        fmt_inner_binary_tree(&self.root, &mut vec![], NULL);
        println!("{BLUE}└───────────────────────────────────{END}");
    }

    /// B木（2-3木）として出力する
    pub fn print_as_btree(&self) {
        println!("{GREEN}┌─ BTree ───────────────────────────{END}");
        fmt_inner_btree(&self.root, self.root.as_ref().map_or(0, |node| node.level));
        println!("{GREEN}└───────────────────────────────────{END}");
    }
}

/// B木（2-3木）として出力する
pub fn print_as_btree<K, M: Monoid>(root: &Node<K, M>)
where
    K: Ord + Debug,
    M::Val: Debug,
{
    println!("{GREEN}┌─ BTree ───────────────────────────{END}");
    fmt_inner_btree(root, root.as_ref().map_or(0, |node| node.level));
    println!("{GREEN}└───────────────────────────────────{END}");
}

/// print recursive
fn fmt_inner_btree<K, M: Monoid>(node: &Node<K, M>, depth: usize)
where
    K: Ord + Debug,
    M::Val: Debug,
{
    if let Some(node) = node.as_ref() {
        fmt_inner_btree(&node.left, depth);
        println!(
            "{GREEN}│{END}{} {:?}",
            "    ".repeat(depth - node.level),
            node
        );
        fmt_inner_btree(&node.right, depth);
    }
}

/// 2分木として出力する
pub fn print_as_binary_tree<K, M: Monoid>(root: &Node<K, M>)
where
    K: Ord + Debug,
    M::Val: Debug,
{
    println!("{BLUE}┌─ BinaryTree ──────────────────────{END}");
    fmt_inner_binary_tree(root, &mut vec![], NULL);
    println!("{BLUE}└───────────────────────────────────{END}");
}

/// print recursive
fn fmt_inner_binary_tree<K, M: Monoid>(
    node: &Node<K, M>,
    fill: &mut Vec<&'static str>,
    last: &'static str,
) where
    K: Ord + Debug,
    M::Val: Debug,
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
        fmt_inner_binary_tree(&node.left, fill, LEFT);
        // 自分を出力
        println!(
            "{BLUE}│{END}{} {:?}",
            fill.iter().fold(String::new(), |s, x| s + x),
            node
        );
        // 右の子
        fmt_inner_binary_tree(&node.right, fill, RIGHT);
        fill.pop();
        // 戻す
        if let Some(tmp) = tmp {
            fill.pop();
            fill.push(tmp);
        }
    }
}
