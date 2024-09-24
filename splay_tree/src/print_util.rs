//! 木を整形して表示するための関数

use std::fmt::Debug;

use crate::node::pointer::NodePtr;

const BLUE: &str = "\x1b[94m";
const END: &str = "\x1b[0m";
const LEFT: &str = " ┌──";
const MID: &str = " │  ";
const RIGHT: &str = " └──";
const NULL: &str = "";
const BLANK: &str = "    ";

/// 2分木として出力する
pub fn print_as_tree<K: Ord + Debug, V: Debug>(root: &NodePtr<K, V>) {
    eprintln!("{BLUE}┌─ BinaryTree ──────────────────────────────────────────{END}");
    fmt_inner_binary_tree(root, &mut vec![], NULL);
    eprintln!("{BLUE}└───────────────────────────────────────────────────────{END}");
}

/// print recursive
fn fmt_inner_binary_tree<K: Ord + Debug, V: Debug>(
    node: &NodePtr<K, V>,
    fill: &mut Vec<&'static str>,
    last: &'static str,
) {
    if let Some(node) = node {
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
        eprintln!(
            "{BLUE}│{END}{} Node {{ key: {:?}, value: {:?} }}",
            fill.iter().fold(String::new(), |s, x| s + x),
            node.borrow().key,
            node.borrow().value,
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
