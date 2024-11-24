//! デバッグ用関数群

use colored::Colorize;

use crate::node::{Internal, Leaf, Node, NodePtr};
use std::fmt::Debug;

const LEFT: &str = "  ┌─";
const MID: &str = "  │ ";
const SEP: &str = "  ├─";
const RIGHT: &str = "  └─";
const NULL: &str = "";
const BLANK: &str = "    ";

impl<const D: usize, K: Debug, V: Debug> Debug for Node<D, K, V>
where
    [(); D + 1]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Internal(Internal {
                keys,
                vals,
                children,
                ..
            }) => f
                .debug_struct("Internal")
                .field("keys", &keys)
                .field("vals", &vals)
                .field("children", &children)
                .finish(),
            Node::Leaf(Leaf { keys, vals, .. }) => f
                .debug_struct("Leaf")
                .field("keys", &keys)
                .field("vals", &vals)
                .finish(),
        }
    }
}

/// 2分木として出力する
pub fn print_as_tree<const D: usize, K: Ord + Debug, V: Debug>(root: &Option<NodePtr<D, K, V>>)
where
    [(); D + 1]:,
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
    [(); D + 1]:,
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

    match &*T.borrow() {
        Node::Internal(Internal {
            keys,
            vals,
            children,
            size,
            ..
        }) => {
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

        Node::Leaf(Leaf {
            keys, vals, size, ..
        }) => {
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
    keys: &[Option<K>; D],
    vals: &[Option<V>; D],
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
