//! デバッグ用関数群

use crate::node::{Node, NodePtr};
use std::fmt::Debug;

const LEFT: &str = "  ┌─";
const MID: &str = "  │ ";
const SEP: &str = "  ├─";
const RIGHT: &str = "  └─";
const NULL: &str = "";
const BLANK: &str = "    ";

impl<K: Debug, V: Debug> Debug for Node<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Internal {
                keys,
                vals,
                children,
                ..
            } => f
                .debug_struct("Internal")
                .field("keys", &keys)
                .field("vals", &vals)
                .field("children", &children)
                .finish(),
            Node::Leaf { keys, vals, .. } => f
                .debug_struct("Leaf")
                .field("keys", &keys)
                .field("vals", &vals)
                .finish(),
        }
    }
}

/// 2分木として出力する
pub fn print_as_tree<K: Ord + Debug, V: Debug>(root: &Option<NodePtr<K, V>>) {
    eprintln!("┌─ BinaryTree ──────────────────────────────────────────");
    dbg_inner(root, &mut vec![], NULL);
    eprintln!("└───────────────────────────────────────────────────────");
}

/// 再帰的に表示
fn dbg_inner<K, V>(root: &Option<NodePtr<K, V>>, fill: &mut Vec<&'static str>, last: &'static str)
where
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
        Node::Internal {
            keys,
            vals,
            children,
            size,
            ..
        } => {
            // 子ノードと値を表示
            for i in 0..*size {
                // 子ノードを表示
                dbg_inner(&children[i], fill, if i == 0 { LEFT } else { SEP });
                // キー，値を表示
                eprintln!(
                    "│{} Node {{ key: {:?}, val: {:?} }}",
                    modify_fill(fill, last, i, size),
                    keys[i].as_ref().unwrap(),
                    vals[i].as_ref().unwrap(),
                );
            }
            // 右の子ノードを表示
            dbg_inner(&children[*size], fill, RIGHT);
        }

        Node::Leaf {
            keys, vals, size, ..
        } => {
            for i in 0..*size {
                // キー，値を表示
                eprintln!(
                    "│{} Node {{ key: {:?}, val: {:?} }}",
                    modify_fill(fill, last, i, size),
                    keys[i].as_ref().unwrap(),
                    vals[i].as_ref().unwrap(),
                );
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

/// 自分の前に表示する文字列を調整する
fn modify_fill(fill: &Vec<&'static str>, last: &'static str, i: usize, size: &usize) -> String {
    if last == LEFT && i != 0 || last == RIGHT && i != size - 1 {
        let mut fill = fill.clone();
        if let Some(prv) = fill.last_mut() {
            *prv = SEP;
        }
        fill.join("")
    } else {
        fill.join("")
    }
}
