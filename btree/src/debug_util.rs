//! デバッグ用関数群

use crate::node::Node;
use std::fmt::Debug;

const GREEN: &str = "\x1b[92m";
const END: &str = "\x1b[0m";

pub trait MyDebug {
    fn dbg(&self);
}

impl<K, V, const DEG: usize> MyDebug for Node<K, V, DEG>
where
    K: Debug,
    V: Debug,
    [(); DEG - 1]:,
{
    /// 木の形でデバッグ出力を行う
    fn dbg(&self) {
        #![cfg(debug_assertions)]
        eprintln!("{GREEN}┌─ BTree ───────────────────────────{END}");
        dbg_inner(&self, 0);
        eprintln!("{GREEN}└───────────────────────────────────{END}");
    }
}

/// 再帰的に表示
pub fn dbg_inner<K, V, const DEG: usize>(root: &Node<K, V, DEG>, depth: usize)
where
    K: Debug,
    V: Debug,
    [(); DEG - 1]:,
{
    match &root {
        Node::Internal {
            keys,
            vals,
            children,
            len,
            ..
        } => {
            // 最も左のノードを表示（少なくとも1つの子は持つ）
            dbg_inner(&children[0].as_ref().unwrap().borrow(), depth + 1);
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
                dbg_inner(&children[i + 1].as_ref().unwrap().borrow(), depth + 1);
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
