use rand::rngs::ThreadRng;
use rand::{self, Rng};
use std::ops::{Deref, DerefMut};
use std::{cmp::Ordering, fmt, mem};

#[derive(Debug)]
pub struct TreapNode<T> {
    pub priority: f64,
    pub value: T,
    pub left: Option<Box<TreapNode<T>>>,
    pub right: Option<Box<TreapNode<T>>>,
}

#[derive(Debug)]
pub struct Treap<T> {
    rng: ThreadRng,
    size: usize,
    pub root: Option<Box<TreapNode<T>>>,
}

impl<T: Ord> Treap<T> {
    pub fn new() -> Self {
        Treap {
            rng: rand::thread_rng(),
            size: 0,
            root: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn search(&mut self, value: &T) -> bool {
        search_inner(value, &self.root)
    }

    pub fn insert(&mut self, value: T) -> bool {
        let res = search_mut(&value, &mut self.root);
        if res.is_none() {
            *res = Some(Box::new(TreapNode {
                priority: self.rng.gen(),
                value,
                left: None,
                right: None,
            }));
            self.size += 1; // 要素数をインクリメント
            true
        } else {
            false
        }
    }

    pub fn discard(&mut self, value: &T) -> bool {
        let res = search_mut(value, &mut self.root);
        if res.as_ref().is_none() {
            false
        } else {
            let root = res.as_deref_mut().unwrap();
            match (root.left.take(), root.right.take()) {
                (None, None) => {
                    *res = None;
                }
                (Some(left), None) => {
                    *res = Some(left);
                }
                (None, Some(right)) => {
                    *res = Some(right);
                }
                (Some(mut left), Some(right)) => {
                    if let Some(mut rightmost) = left.rightmost_child() {
                        rightmost.left = Some(left);
                        rightmost.right = Some(right);
                        *res = Some(rightmost);
                    } else {
                        left.right = Some(right);
                        *res = Some(left);
                    }
                }
            };
            self.size -= 1; // 要素数をデクリメント
            true
        }
    }
}

impl<T: Ord + fmt::Debug> Treap<T> {
    pub fn pretty_print(&self) {
        pretty_print_inner(&self.root, 0);
    }
}

/// 整形して表示
fn pretty_print_inner<K: Ord + fmt::Debug>(node: &Option<Box<TreapNode<K>>>, depth: usize) {
    match node {
        Some(ref node) => {
            pretty_print_inner(&node.left, depth + 2);
            println!(
                "{}{{p:{:.2}, val:{:?}}}",
                " ".repeat(depth * 2),
                node.priority,
                node.value
            );
            pretty_print_inner(&node.right, depth + 2);
        }
        None => {}
    }
}

/// keyを検索する
fn search_inner<T: Ord>(value: &T, root: &Option<Box<TreapNode<T>>>) -> bool {
    if root.is_none() {
        return false;
    }
    let node = root.as_ref().unwrap();
    match value.cmp(&node.value) {
        Ordering::Equal => true,
        Ordering::Less => search_inner(value, &node.left),
        Ordering::Greater => search_inner(value, &node.right),
    }
}

/// keyを挿入するべき位置にあるノードを返す
fn search_mut<'a, T: Ord>(
    value: &T,
    root: &'a mut Option<Box<TreapNode<T>>>,
) -> &'a mut Option<Box<TreapNode<T>>> {
    if root.is_none() {
        return root;
    }
    match value.cmp(&root.as_ref().unwrap().value) {
        Ordering::Equal => root,
        Ordering::Less => search_mut(value, &mut root.as_mut().unwrap().left),
        Ordering::Greater => search_mut(value, &mut root.as_mut().unwrap().right),
    }
}

impl<T: Ord> TreapNode<T> {
    fn rightmost_child(&mut self) -> Option<Box<Self>> {
        match self.right {
            Some(ref mut right) => {
                if let Some(node) = right.rightmost_child() {
                    // 右の子に右の子が存在する場合
                    Some(node)
                } else {
                    // 右の子に右の子が存在しない場合
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = mem::replace(&mut r.left, None);
                    }
                    r
                }
            }
            None => None,
        }
    }
}

/// ノードの右回転を行う
fn rotate_right<T>(root: Option<Box<TreapNode<T>>>) -> Option<Box<TreapNode<T>>> {
    if let Some(mut root) = root {
        if let Some(mut new_root) = root.left {
            root.left = new_root.right;
            new_root.right = Some(root);
            Some(new_root)
        } else {
            Some(root)
        }
    } else {
        None
    }
}

/// ノードの右回転を行う
fn rotate_left<T>(root: Option<Box<TreapNode<T>>>) -> Option<Box<TreapNode<T>>> {
    if let Some(mut root) = root {
        if let Some(mut new_root) = root.right {
            root.right = new_root.left;
            new_root.left = Some(root);
            Some(new_root)
        } else {
            Some(root)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut root = Some(Box::new(TreapNode {
            priority: 0.0,
            value: 4,
            left: Some(Box::new(TreapNode {
                priority: 0.0,
                value: 2,
                left: Some(Box::new(TreapNode {
                    priority: 0.0,
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreapNode {
                    priority: 0.0,
                    value: 3,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreapNode {
                priority: 0.0,
                value: 5,
                left: None,
                right: None,
            })),
        }));

        println!("----- 回転前 -----");
        pretty_print_inner(&root, 0);

        // ## 右回転のテスト
        // 右回転
        root = rotate_right(root);

        println!("----- 右回転 -----");
        pretty_print_inner(&root, 0);

        // さらに右回転
        root = rotate_right(root);

        println!("----- 右回転 -----");
        pretty_print_inner(&root, 0);

        // さらに右回転
        root = rotate_right(root);

        println!("----- 右回転 -----");
        pretty_print_inner(&root, 0);

        // ## 左回転のテスト
        // 左回転
        root = rotate_left(root);

        println!("----- 左回転 -----");
        pretty_print_inner(&root, 0);

        // さらに左回転
        root = rotate_left(root);

        println!("----- 左回転 -----");
        pretty_print_inner(&root, 0);

        // さらに左回転
        root = rotate_left(root);

        println!("----- 左回転 -----");
        pretty_print_inner(&root, 0);
    }
}
