//! ノードの構造体

use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::Debug,
    mem,
    rc::{Rc, Weak},
};

/// ノードの構造体
pub struct Node<K: Ord, V> {
    pub key: K,
    pub value: V,
    pub parent: Option<Weak<RefCell<Node<K, V>>>>,
    pub left: Option<Rc<RefCell<Node<K, V>>>>,
    pub right: Option<Rc<RefCell<Node<K, V>>>>,
}

/// ノードのポインタ
pub type NodePtr<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

/// 親ノードのポインタ
pub type ParentPtr<K, V> = Option<Weak<RefCell<Node<K, V>>>>;

impl<K: Ord, V> Node<K, V> {
    /// 葉ノードを作成する
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            parent: None,
            left: None,
            right: None,
        }
    }

    /// ノードのポインタを確保する
    pub fn node_ptr(key: K, value: V) -> NodePtr<K, V> {
        Some(Rc::new(RefCell::new(Self::new(key, value))))
    }
}

/// nodeを根とする木に(key, value)を挿入する
/// - すでに同じキーが存在した場合，その値を置き換える
///
/// **引数**
/// - node: 挿入対象のノード
/// - key: キー
/// - value: 値
///
/// **戻り値**
/// - NodePtr<K, V>: 挿入後のノード
/// - Option<V>: 置き換えられた値
fn insert<K: Ord, V: Clone>(node: NodePtr<K, V>, key: K, value: V) -> (NodePtr<K, V>, Option<V>) {
    let Some(node) = node else {
        return (Node::node_ptr(key, value), None);
    };

    // キーの比較
    let comp = key.cmp(&node.as_ref().borrow().key);

    match comp {
        Ordering::Less => {
            // 左の子に挿入
            let left = node.borrow_mut().left.take();
            let (mut new_left, old_value) = insert(left, key, value);

            // new_leftの親の更新
            let node_ptr_weak = Rc::downgrade(&node);
            new_left.as_mut().unwrap().borrow_mut().parent = Some(node_ptr_weak);

            // 子を戻す
            node.borrow_mut().left = new_left;

            (Some(node), old_value)
        }
        Ordering::Equal => {
            // valueを置き換える
            let old_value = mem::replace(&mut node.borrow_mut().value, value);

            (Some(node), Some(old_value))
        }
        Ordering::Greater => {
            // 左の子に挿入
            let right = node.borrow_mut().right.take();
            let (mut new_right, old_value) = insert(right, key, value);

            // new_rightの親の更新
            let node_ptr_weak = Rc::downgrade(&node);
            new_right.as_mut().unwrap().borrow_mut().parent = Some(node_ptr_weak);

            // 子を戻す
            node.borrow_mut().right = new_right;

            (Some(node), old_value)
        }
    }
}

/// rootを根とする木で，xに一致するキーをもつノードの参照を返す
pub fn find<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePtr<K, V>) {
    let mut node = root.clone();
    while let Some(inner) = node.clone() {
        let comp = x.cmp(&inner.borrow().key);
        match comp {
            Ordering::Less => node = inner.borrow().left.clone(),
            Ordering::Equal => break,
            Ordering::Greater => node = inner.borrow().right.clone(),
        }
    }
    (root, node)
}

/// nodeを1つ上に持ってくるように回転する
fn rotate<K: Ord, V>(node: NodePtr<K, V>) -> NodePtr<K, V> {
    match NodeState::get(&node) {
        NodeState::Nil | NodeState::Root => node,
        NodeState::LeftChild => {
            let inner = node?;
            let mut right = inner.borrow_mut().right.take();
            let par = inner.borrow().parent.clone()?;

            // 親の左の子←自分の右の子
            if let Some(right) = &mut right {
                right.borrow_mut().parent = Some(par.clone());
            }
            let par = par.upgrade()?;
            par.borrow_mut().left = right;

            // 自分の親←親の親
            let par_state = NodeState::get(&Some(par.clone()));
            let mut parpar = par.borrow_mut().parent.take();
            if let Some(parpar) = &mut parpar {
                match par_state {
                    NodeState::LeftChild => {
                        parpar.upgrade()?.borrow_mut().left = Some(inner.clone());
                    }
                    NodeState::RightChild => {
                        parpar.upgrade()?.borrow_mut().right = Some(inner.clone());
                    }
                    _ => (),
                }
            }
            inner.borrow_mut().parent = parpar;

            // 自分の右の子←親
            par.borrow_mut().parent = Some(Rc::downgrade(&inner));
            inner.borrow_mut().right = Some(par);

            Some(inner)
        }
        NodeState::RightChild => {
            let inner = node?;
            let mut left = inner.as_ref().borrow_mut().left.take();
            let par = inner.borrow().parent.clone()?;

            // 親の右の子←自分の左の子
            if let Some(left) = &mut left {
                left.borrow_mut().parent = Some(par.clone());
            }
            let par = par.upgrade()?;
            par.borrow_mut().right = left;

            // 自分の親←親の親
            let par_state = NodeState::get(&Some(par.clone()));
            let mut parpar = par.borrow_mut().parent.take();
            if let Some(parpar) = &mut parpar {
                match par_state {
                    NodeState::LeftChild => {
                        parpar.upgrade()?.borrow_mut().left = Some(inner.clone());
                    }
                    NodeState::RightChild => {
                        parpar.upgrade()?.borrow_mut().right = Some(inner.clone());
                    }
                    _ => (),
                }
            }
            inner.borrow_mut().parent = parpar;

            // 自分の左の子←親
            par.borrow_mut().parent = Some(Rc::downgrade(&inner));
            inner.borrow_mut().left = Some(par);

            Some(inner)
        }
    }
}

/// 親のRc参照を取得する
fn get_parent<K: Ord, V>(node: &NodePtr<K, V>) -> NodePtr<K, V> {
    node.clone()?
        .borrow()
        .parent
        .as_ref()
        .map(|p| p.upgrade().unwrap())
}

/// スプレー操作によりnodeを根に移動し，新たな根を返す
fn splay<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    let mut state = NodeState::get(&node);
    while state.is_child() {
        // 親頂点の状態
        let par_state = NodeState::get_from_weak(&node.as_ref()?.borrow().parent);

        match (state, par_state) {
            // zig
            (NodeState::LeftChild | NodeState::RightChild, NodeState::Root) => {
                node = rotate(node);
            }
            // zig-zig
            (NodeState::LeftChild, NodeState::RightChild)
            | (NodeState::RightChild, NodeState::LeftChild) => {
                node = rotate(node);
                node = rotate(node);
            }
            // zig-zag
            (NodeState::LeftChild, NodeState::LeftChild)
            | (NodeState::RightChild, NodeState::RightChild) => {
                // 親を先にrotate（オブジェクトをdropさせないため，変数に代入する）
                let _par = rotate(get_parent(&node));
                node = rotate(node);
            }
            _ => unreachable!(),
        }

        state = NodeState::get(&node);
    }
    node
}

/// ノードの状態を調べる
#[derive(Debug, PartialEq)]
pub enum NodeState {
    /// ノードが存在しない
    Nil,
    /// 根ノード（親を持たない）
    Root,
    /// 親の左の子
    LeftChild,
    /// 親の右の子
    RightChild,
}

impl NodeState {
    /// 与えられたノードが
    /// - 空のノード
    /// - 根ノード
    /// - 親の左の子
    /// - 親の右の子
    ///
    /// のどれかを判定する．
    fn get<K: Ord, V>(node: &NodePtr<K, V>) -> Self {
        let Some(inner) = node else {
            return NodeState::Nil;
        };
        let Some(par) = inner.borrow().parent.clone() else {
            return NodeState::Root;
        };
        // 左の子である場合
        let par = par.upgrade().unwrap();
        if par
            .borrow()
            .left
            .as_ref()
            .is_some_and(|left| Rc::ptr_eq(left, inner))
        {
            NodeState::LeftChild
        } else {
            NodeState::RightChild
        }
    }

    fn get_from_weak<K: Ord, V>(node: &ParentPtr<K, V>) -> Self {
        let node = node.as_ref().map(|p| p.upgrade().unwrap());
        Self::get(&node)
    }

    /// 子頂点であるかを判定する
    fn is_child(&self) -> bool {
        matches!(self, Self::LeftChild | Self::RightChild)
    }
}

/// 次に大きい値をもつノードを返す
fn next<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    if node.is_none() {
        return None;
    }

    // 右の子がいる ⇒ 右の子孫の最小値
    if let Some(mut nxt) = node.as_ref()?.borrow().right.clone() {
        while nxt.borrow().left.is_some() {
            let left = nxt.borrow().left.clone()?;
            nxt = left;
        }
        return Some(nxt);
    }

    // 親をたどる
    let mut state = NodeState::get(&node);
    while state.is_child() {
        match state {
            NodeState::LeftChild => {
                return get_parent(&node);
            }
            NodeState::RightChild => {
                node = get_parent(&node);
            }
            _ => unreachable!(),
        }
        state = NodeState::get(&node);
    }

    None
}

impl<K: Ord + Debug, V: Debug> Debug for Node<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.left, &self.right) {
            (None, None) => f
                .debug_struct("Node")
                .field(&"key", &self.key)
                .field(&"value", &self.value)
                .finish(),
            (Some(_), None) => f
                .debug_struct("Node")
                .field(&"key", &self.key)
                .field(&"value", &self.value)
                .field(&"left", &self.left)
                .finish(),
            (None, Some(_)) => f
                .debug_struct("Node")
                .field(&"key", &self.key)
                .field(&"value", &self.value)
                .field(&"right", &self.right)
                .finish(),
            (Some(_), Some(_)) => f
                .debug_struct("Node")
                .field(&"key", &self.key)
                .field(&"value", &self.value)
                .field(&"left", &self.left)
                .field(&"right", &self.right)
                .finish(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        node::{insert, rotate, splay, NodeState},
        print_util::print_as_binary_tree,
    };

    use super::{find, next, Node};

    #[test]
    fn test_create_tree() {
        let root = Node::new(0, "root");

        eprintln!("{:?}", root);
    }

    #[test]
    fn test_insert() {
        let mut root = None;
        print_as_binary_tree(&root);

        (root, _) = insert(root, 5, "first");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 15, "second");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 1, "third");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 3, "forth");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 30, "fifth");
        print_as_binary_tree(&root);

        (root, _) = insert(root, 15, "sixth");
        print_as_binary_tree(&root);
    }

    #[test]
    fn test_find() {
        let mut root = None;
        (root, _) = insert(root, 5, "first");
        (root, _) = insert(root, 15, "second");
        (root, _) = insert(root, 1, "third");
        (root, _) = insert(root, 3, "forth");
        (root, _) = insert(root, 30, "fifth");

        let find_5;
        (root, find_5) = find(root, &5);
        print_as_binary_tree(&root);
        println!("{:?}", &find_5);

        let find_20;
        (root, find_20) = find(root, &2);
        print_as_binary_tree(&root);
        println!("{:?}", &find_20);

        let find_15;
        print_as_binary_tree(&root);
        (root, find_15) = find(root, &15);
        println!("{:?}", &find_15);

        (root, _) = insert(root, 20, "sixth");
        print_as_binary_tree(&root);
        println!("{:?}", &find_15);
    }

    #[test]
    fn test_nodestate() {
        let mut root = None;
        (root, _) = insert(root, 5, "first");
        (root, _) = insert(root, 15, "second");
        (root, _) = insert(root, 1, "third");
        (root, _) = insert(root, 3, "forth");
        (root, _) = insert(root, 30, "fifth");

        print_as_binary_tree(&root);

        let find_1;
        (root, find_1) = find(root, &1);
        println!("find_1 = {:?}", NodeState::get(&find_1));

        let find_3;
        (root, find_3) = find(root, &3);
        println!("find_3 = {:?}", NodeState::get(&find_3));

        let find_5;
        (root, find_5) = find(root, &5);
        println!("find_5 = {:?}", NodeState::get(&find_5));

        let find_15;
        (root, find_15) = find(root, &15);
        println!("find_15 = {:?}", NodeState::get(&find_15));

        let find_20;
        (root, find_20) = find(root, &20);
        println!("find_20 = {:?}", NodeState::get(&find_20));

        let find_30;
        (root, find_30) = find(root, &30);
        println!("find_30 = {:?}", NodeState::get(&find_30));

        (root, _) = insert(root, 20, "sixth");
        print_as_binary_tree(&root);

        let find_20;
        (root, find_20) = find(root, &20);
        println!("find_20 = {:?}", NodeState::get(&find_20));
    }

    #[test]
    fn test_rotate_right() {
        let mut root = None;
        (root, _) = insert(root, 5, "first");
        (root, _) = insert(root, 15, "second");
        (root, _) = insert(root, 1, "third");
        (root, _) = insert(root, 3, "forth");
        (root, _) = insert(root, 30, "fifth");

        print_as_binary_tree(&root);

        let find_5;
        (root, find_5) = find(root, &5);
        println!("find_5 = {:?}", NodeState::get(&find_5));

        // rootを回転
        println!("> rotate at root");
        root = rotate(root);

        print_as_binary_tree(&root);

        {
            let mut find_1;
            (root, find_1) = find(root, &1);
            println!("find_1 = {:?}", NodeState::get(&find_1));

            // 回転
            println!("> rotate 1");
            find_1 = rotate(find_1);

            print_as_binary_tree(&root);
            print_as_binary_tree(&find_1);

            println!("root = {:?}", NodeState::get(&root));
            println!("find_1 = {:?}", NodeState::get(&find_1));

            root = find_1;
        }

        {
            let mut find_3;
            (root, find_3) = find(root, &3);
            println!("find_3 = {:?}", NodeState::get(&find_3));

            // 30を回転
            println!("> rotate 3");
            find_3 = rotate(find_3);

            print_as_binary_tree(&root);

            println!("root = {:?}", NodeState::get(&root));
            println!("find_3 = {:?}", NodeState::get(&find_3));
        }
    }

    #[test]
    fn test_rotate_left() {
        let mut root = None;
        (root, _) = insert(root, 5, "first");
        (root, _) = insert(root, 15, "second");
        (root, _) = insert(root, 1, "third");
        (root, _) = insert(root, 3, "forth");
        (root, _) = insert(root, 30, "fifth");

        print_as_binary_tree(&root);

        {
            let mut find_30;
            (root, find_30) = find(root, &30);
            println!("find_30 = {:?}", NodeState::get(&find_30));

            // 回転
            println!("> rotate 30");
            find_30 = rotate(find_30);

            print_as_binary_tree(&root);
            print_as_binary_tree(&find_30);

            println!("root = {:?}", NodeState::get(&root));
            println!("find_30 = {:?}", NodeState::get(&find_30));
        }

        {
            let mut find_30;
            (root, find_30) = find(root, &30);
            println!("find_30 = {:?}", NodeState::get(&find_30));

            // 回転
            println!("> rotate 30");
            find_30 = rotate(find_30);

            print_as_binary_tree(&root);
            print_as_binary_tree(&find_30);

            println!("root = {:?}", NodeState::get(&root));
            println!("find_30 = {:?}", NodeState::get(&find_30));

            root = find_30;
        }
    }

    #[test]
    fn test_splay() {
        let mut root = None;

        (root, _) = insert(root, 1, "first");
        (root, _) = insert(root, 3, "second");
        (root, _) = insert(root, 4, "third");
        (root, _) = insert(root, 9, "forth");
        (root, _) = insert(root, 2, "fifth");

        print_as_binary_tree(&root);

        let node;
        (root, node) = find(root, &4);

        root = splay(node);

        print_as_binary_tree(&root);

        let node;
        (root, node) = find(root, &1);

        root = splay(node);

        print_as_binary_tree(&root);

        let node;
        (root, node) = find(root, &9);

        root = splay(node);

        print_as_binary_tree(&root);
    }

    #[test]
    fn test_next() {
        let mut root = None;

        for i in [7, 4, 100, 0, 6, -1, 33, 21] {
            (root, _) = insert(root, i, i);
        }

        print_as_binary_tree(&root);

        let mut nxt;
        (root, nxt) = find(root, &-1);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");

        nxt = next(nxt);
        println!("> {nxt:?}");
    }
}
