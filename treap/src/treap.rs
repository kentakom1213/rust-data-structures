use rand::rngs::ThreadRng;
use rand::{self, Rng};
use std::fmt::Debug;
use std::mem::swap;
use std::{cmp::Ordering, fmt};

#[derive(Debug)]
struct TreapNode<T> {
    pub priority: f64,
    pub value: T,
    pub left: Option<Box<TreapNode<T>>>,
    pub right: Option<Box<TreapNode<T>>>,
}

#[derive(Debug)]
pub struct Treap<T> {
    rng: ThreadRng,
    size: usize,
    root: Option<Box<TreapNode<T>>>,
}

impl<T: Ord + Clone> Treap<T> {
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
        let root = self.root.take();
        let (new_root, is_inserted) = insert_inner(value, self.rng.gen(), root);
        self.root = new_root;
        if is_inserted {
            self.size += 1;
            true
        } else {
            false
        }
    }

    pub fn discard(&mut self, value: &T) -> bool {
        let root = self.root.take();
        let (new_root, is_deleted) = delete_inner(value, root);
        self.root = new_root;
        if is_deleted {
            self.size -= 1;
            true
        } else {
            false
        }
    }

    pub fn lower_bound(&self, value: &T) -> Option<&T> {
        let mut root = &self.root;
        let mut last = &None;
        while root.is_some() {
            match value.cmp(&root.as_ref().unwrap().value) {
                Ordering::Less | Ordering::Equal => {
                    last = root;
                    root = &root.as_ref().unwrap().left;
                }
                Ordering::Greater => {
                    root = &root.as_ref().unwrap().right;
                }
            }
        }
        if let Some(last) = last {
            Some(&last.as_ref().value)
        } else {
            None
        }
    }

    pub fn upper_bound(&self, value: &T) -> Option<&T> {
        let mut root = &self.root;
        let mut last = &None;
        while root.is_some() {
            match value.cmp(&root.as_ref().unwrap().value) {
                Ordering::Less => {
                    last = root;
                    root = &root.as_ref().unwrap().left;
                }
                Ordering::Equal | Ordering::Greater => {
                    root = &root.as_ref().unwrap().right;
                }
            }
        }
        if let Some(last) = last {
            Some(&last.as_ref().value)
        } else {
            None
        }
    }
}

impl<T: Ord + fmt::Debug> Treap<T> {
    /// 整形して表示する
    pub fn pretty_print(&self) {
        pretty_print_inner(&self.root, 0);
    }
}

/// 再帰的に表示
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

/// 指定されたキーを削除し，新しい根を返す（所有権を受け取る）
fn delete_inner<T: Ord>(
    value: &T,
    root: Option<Box<TreapNode<T>>>,
) -> (Option<Box<TreapNode<T>>>, bool) {
    if let Some(mut root) = root {
        match value.cmp(&root.value) {
            Ordering::Equal => {
                // 値が等しい場合，その要素を葉に持っていき，削除する
                match (root.left.is_some(), root.right.is_some()) {
                    (false, false) => (None, true),
                    (false, true) => {
                        root = rotate_left(Some(root)).unwrap();
                        // 左部分木からvalueを削除
                        let left = root.left.take();
                        let (new_left, _) = delete_inner(value, left);
                        root.left = new_left;
                        (Some(root), true)
                    }
                    (true, _) => {
                        root = rotate_right(Some(root)).unwrap();
                        // 右部分木からvalueを削除
                        let right = root.right.take();
                        let (new_right, _) = delete_inner(value, right);
                        root.right = new_right;
                        (Some(root), true)
                    }
                }
            }
            Ordering::Less => {
                let left = root.left.take();
                let (mut new_left, is_deleted) = delete_inner(value, left);
                swap(&mut root.left, &mut new_left);
                (Some(root), is_deleted)
            }
            Ordering::Greater => {
                let right = root.right.take();
                let (mut new_right, is_deleted) = delete_inner(value, right);
                swap(&mut root.right, &mut new_right);
                (Some(root), is_deleted)
            }
        }
    } else {
        (None, false)
    }
}

/// keyを挿入するべき位置にあるノードを返す（所有権を受け取る）
fn insert_inner<T: Ord>(
    value: T,
    priority: f64,
    mut root: Option<Box<TreapNode<T>>>,
) -> (Option<Box<TreapNode<T>>>, bool) {
    if let Some(mut root) = root {
        match value.cmp(&root.value) {
            Ordering::Equal => (Some(root), false),
            Ordering::Less => {
                let left = root.left.take();
                let (mut child, is_inserted) = insert_inner(value, priority, left);
                swap(&mut root.left, &mut child);
                if is_inserted {
                    if root.priority > root.left.as_deref().unwrap().priority {
                        // 親のpriorityの方が大きい場合，右回転を行う
                        let new_root = rotate_right(Some(root));
                        (new_root, true)
                    } else {
                        // それ以外の場合，回転を行わない
                        (Some(root), true)
                    }
                } else {
                    (Some(root), false)
                }
            }
            Ordering::Greater => {
                let right = root.right.take();
                let (mut child, is_inserted) = insert_inner(value, priority, right);
                swap(&mut root.right, &mut child);
                if is_inserted {
                    if root.priority > root.right.as_deref().unwrap().priority {
                        // 親のpriorityの方が大きい場合，左回転を行う
                        let new_root = rotate_left(Some(root));
                        (new_root, true)
                    } else {
                        // それ以外の場合，回転を行わない
                        (Some(root), true)
                    }
                } else {
                    (Some(root), false)
                }
            }
        }
    } else {
        // ノードを挿入
        root = Some(Box::new(TreapNode {
            priority,
            value,
            left: None,
            right: None,
        }));
        (root, true)
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
    fn test_insert_greater() {
        let mut tree = Treap::new();

        // 10を挿入
        println!("--- insert 10 ---");
        tree.insert(10);
        tree.pretty_print();

        // 5を挿入
        println!("--- insert 5 ---");
        tree.insert(5);
        tree.pretty_print();

        // 0を挿入
        println!("--- insert 0 ---");
        tree.insert(0);
        tree.pretty_print();
    }

    #[test]
    fn test_insert_less() {
        let mut tree = Treap::new();

        // 0を挿入
        println!("--- insert 0 ---");
        tree.insert(0);
        tree.pretty_print();

        // 5を挿入
        println!("--- insert 5 ---");
        tree.insert(5);
        tree.pretty_print();

        // 10を挿入
        println!("--- insert 10 ---");
        tree.insert(10);
        tree.pretty_print();
    }

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

        println!("{:?}", &root);

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
