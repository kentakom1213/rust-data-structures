use std::rc::Rc;

use super::{
    state::{get_parent, NodeState},
    NodePtr,
};

/// nodeを1つ上に持ってくるように回転する
pub fn rotate<K: Ord, V>(node: NodePtr<K, V>) -> NodePtr<K, V> {
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

/// スプレー操作によりnodeを根に移動し，新たな根を返す
pub fn splay<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
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

#[cfg(test)]
mod test_splay {
    use crate::{
        node::{
            insert::{find, insert},
            splay::rotate,
            state::NodeState,
        },
        print_util::print_as_binary_tree,
    };

    use super::splay;

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
}
