//! デバッグ用関数群

use colored::Colorize;

use crate::node::{BTreeNode, NodePtr};
use std::fmt::Debug;

const LEFT: &str = "  ┌─";
const MID: &str = "  │ ";
const SEP: &str = "  ├─";
const RIGHT: &str = "  └─";
const NULL: &str = "";
const BLANK: &str = "    ";

impl<const D: usize, K: Debug, V: Debug> Debug for BTreeNode<D, K, V>
where
    [(); 2 * D - 1]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let BTreeNode {
            size,
            keys,
            vals,
            children,
            ..
        } = self;

        match children {
            Some(_) => f
                .debug_struct("Internal")
                .field("size", &size)
                .field("keys", &keys)
                .field("vals", &vals)
                // .field("children", &children)
                .finish(),
            None => f
                .debug_struct("Leaf")
                .field("size", &size)
                .field("keys", &keys)
                .field("vals", &vals)
                .finish(),
        }
    }
}

/// 2分木として出力する
pub fn print_as_tree<const D: usize, K: Ord + Debug, V: Debug>(root: &Option<NodePtr<D, K, V>>)
where
    [(); 2 * D - 1]:,
{
    eprintln!(
        "{}",
        "┌─ B-Tree ──────────────────────────────────────────────".blue()
    );
    dbg_inner(root, &mut vec![], NULL);
    eprintln!(
        "{}",
        "└───────────────────────────────────────────────────────".blue()
    );
}

/// 再帰的に表示
fn dbg_inner<const D: usize, K, V>(
    root: &Option<NodePtr<D, K, V>>,
    fill: &mut Vec<&'static str>,
    last: &'static str,
) where
    [(); 2 * D - 1]:,
    K: Debug,
    V: Debug,
{
    let Some(T) = root else {
        return;
    };

    // 表示の調整
    let mut tmp = None;
    if fill.last().is_some_and(|x| x == &last && x != &SEP) {
        tmp = fill.pop();
        fill.push(BLANK);
    } else if fill.last().is_some_and(|x| x != &NULL && x != &BLANK) {
        tmp = fill.pop();
        fill.push(MID);
    }
    fill.push(last);

    match T.as_ref() {
        BTreeNode {
            keys,
            vals,
            children: Some(children),
            size,
            ..
        } => {
            // 子ノードと値を表示
            for i in 0..*size {
                // 子ノードを表示
                dbg_inner(&children[i], fill, if i == 0 { LEFT } else { SEP });
                // キー，値を表示
                print_node(keys, vals, fill, last, i, size);
            }
            // 右の子ノードを表示
            dbg_inner(&children[*size], fill, RIGHT);
        }

        BTreeNode {
            keys, vals, size, ..
        } => {
            for i in 0..*size {
                // キー，値を表示
                print_node(keys, vals, fill, last, i, size);
            }
        }
    }

    // 戻す
    fill.pop();
    if let Some(tmp) = tmp {
        fill.pop();
        fill.push(tmp);
    }
}

/// ノードを出力する
fn print_node<const D: usize, K: Debug, V: Debug>(
    keys: &[Option<K>; 2 * D - 1],
    vals: &[Option<V>; 2 * D - 1],
    fill: &Vec<&'static str>,
    last: &'static str,
    i: usize,
    size: &usize,
) {
    let fill = if last == LEFT && i != 0 || last == RIGHT && i != size - 1 {
        let mut fill = fill.clone();
        if let Some(prv) = fill.last_mut() {
            *prv = SEP;
        }
        fill.join("")
    } else {
        fill.join("")
    };

    // キー，値を表示
    eprintln!(
        "{} {} Node {{ key: {:?}, val: {:?} }}",
        "│".blue(),
        fill,
        keys[i].as_ref().unwrap(),
        vals[i].as_ref().unwrap(),
    );
}
