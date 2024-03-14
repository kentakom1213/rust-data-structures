//! デバッグ用関数群

use crate::node::{BTreeNode, Node};
use std::fmt::Debug;

const GREEN: &str = "\x1b[92m";
const END: &str = "\x1b[0m";

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

/// 再帰的に表示
pub fn dbg_inner<K, V>(root: &BTreeNode<K, V>, depth: usize)
where
    K: Debug,
    V: Debug,
{
    let Some(T) = root else {
        return;
    };
    match &*T.borrow() {
        Node::Internal {
            keys,
            vals,
            children,
            len,
            ..
        } => {
            // 最も左のノードを表示（少なくとも1つの子は持つ）
            dbg_inner(&children[0], depth + 1);
            // 子ノードと値を表示
            for i in 0..*len {
                // キー，値を表示
                eprintln!(
                    "{GREEN}│{END}{} [key: {:?}, val: {:?}]",
                    "  ".repeat(depth),
                    keys[i],
                    vals[i],
                );
                // 右の子ノードを表示
                dbg_inner(&children[i + 1], depth + 1);
            }
        }
        Node::Leaf {
            keys, vals, len, ..
        } => {
            for i in 0..*len {
                // キー，値を表示
                eprintln!(
                    "{GREEN}│{END}{} [key: {:?}, val: {:?}]",
                    "  ".repeat(depth),
                    keys[i],
                    vals[i],
                );
            }
        }
    }
}
