use std::{fmt::Debug, rc::Rc};

use super::{
    pointer::{NodeOps, ParentOps},
    state::NodeState,
    NodePtr,
};

/// nodeを1つ上に持ってくるように回転する
pub fn rotate<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    match node.get_state() {
        NodeState::Nil | NodeState::Root => node,
        NodeState::LeftChild => {
            let mut right = node.right_mut()?.take();
            let par = node.parent()?.clone();

            // 親の左の子←自分の右の子
            if let Some(mut right_parent) = right.parent_mut() {
                *right_parent = par.clone();
            }
            let mut par = par.to_strong_ptr();
            *par.left_mut()? = right;

            // 自分の親←親の親
            let par_state = par.get_state();
            let mut parpar = par.take_parent_strong();

            match par_state {
                NodeState::LeftChild => {
                    *parpar.left_mut()? = node.clone();
                }
                NodeState::RightChild => {
                    *parpar.right_mut()? = node.clone();
                }
                _ => (),
            }

            *node.parent_mut()? = parpar.map(|f| Rc::downgrade(&f));

            // 自分の右の子←親
            *par.parent_mut()? = node.to_weak_ptr();
            node.right_mut()?.replace(par?);

            node
        }
        NodeState::RightChild => {
            let mut left = node.left_mut()?.take();
            let par = node.parent()?.clone();

            // 親の右の子←自分の左の子
            if let Some(mut left_parent) = left.parent_mut() {
                *left_parent = par.clone();
            }
            let mut par = par.to_strong_ptr();
            *par.right_mut()? = left;

            // 自分の親←親の親
            let par_state = par.get_state();
            let mut parpar = par.take_parent_strong();

            match par_state {
                NodeState::LeftChild => {
                    *parpar.left_mut()? = node.clone();
                }
                NodeState::RightChild => {
                    *parpar.right_mut()? = node.clone();
                }
                _ => (),
            }

            *node.parent_mut()? = parpar.map(|f| Rc::downgrade(&f));

            // 自分の左の子←親
            *par.parent_mut()? = node.to_weak_ptr();
            node.left_mut()?.replace(par?);

            node
        }
    }
}

/// スプレー操作によりnodeを根に移動し，新たな根を返す
pub fn splay<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    while node.is_child() {
        // 頂点の状態
        let state = node.get_state();
        // 親頂点の状態
        let par_state = node.get_parent_ptr().get_state();

        match (state, par_state) {
            (NodeState::Root, _) => {
                eprintln!("root: {:?}", &node.parent());
                break;
            }
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
                let _par = rotate(node.get_parent_ptr());
                node = rotate(node);
            }
            _ => unreachable!(),
        }
    }
    node
}

#[cfg(test)]
mod test_splay {
    use crate::{
        node::{find::find, insert::insert_single, pointer::NodeOps, splay::rotate},
        print_util::print_as_binary_tree,
    };

    use super::splay;

    #[test]
    fn test_rotate_right() {
        let mut root = None;
        (root, _, _) = insert_single(root, 5, "first");
        (root, _, _) = insert_single(root, 15, "second");
        (root, _, _) = insert_single(root, 1, "third");
        (root, _, _) = insert_single(root, 3, "forth");
        (root, _, _) = insert_single(root, 30, "fifth");

        print_as_binary_tree(&root);

        let find_5 = find(&root, &5);
        println!("find_5 = {:?}", find_5.get_state());

        // rootを回転
        println!("> rotate at root");
        root = rotate(root);

        print_as_binary_tree(&root);

        {
            let mut find_1 = find(&root, &1);
            println!("find_1 = {:?}", find_1.get_state());

            // 回転
            println!("> rotate 1");
            find_1 = rotate(find_1);

            println!("root = {:?}", root.get_state());
            println!("find_1 = {:?}", find_1.get_state());

            root = find_1;

            print_as_binary_tree(&root);
        }

        {
            let mut find_3 = find(&root, &3);
            println!("find_3 = {:?}", find_3.get_state());

            // 30を回転
            println!("> rotate 3");
            find_3 = rotate(find_3);

            print_as_binary_tree(&root);

            println!("root = {:?}", root.get_state());
            println!("find_3 = {:?}", find_3.get_state());
        }
    }

    #[test]
    fn test_rotate_left() {
        let mut root = None;
        (root, _, _) = insert_single(root, 5, "first");
        (root, _, _) = insert_single(root, 15, "second");
        (root, _, _) = insert_single(root, 1, "third");
        (root, _, _) = insert_single(root, 3, "forth");
        (root, _, _) = insert_single(root, 30, "fifth");

        print_as_binary_tree(&root);

        {
            let mut find_30 = find(&root, &30);
            println!("find_30 = {:?}", find_30.get_state());

            // 回転
            println!("> rotate 30");
            find_30 = rotate(find_30);

            print_as_binary_tree(&root);
            print_as_binary_tree(&find_30);

            println!("root = {:?}", root.get_state());
            println!("find_30 = {:?}", find_30.get_state());
        }

        {
            let mut find_30 = find(&root, &30);
            println!("find_30 = {:?}", find_30.get_state());

            // 回転
            println!("> rotate 30");
            find_30 = rotate(find_30);

            print_as_binary_tree(&root);
            print_as_binary_tree(&find_30);

            println!("root = {:?}", root.get_state());
            println!("find_30 = {:?}", find_30.get_state());

            root = find_30;
        }
    }

    #[test]
    fn test_splay() {
        let mut root = None;

        (root, _, _) = insert_single(root, 1, "first");
        (root, _, _) = insert_single(root, 3, "second");
        (root, _, _) = insert_single(root, 4, "third");
        (root, _, _) = insert_single(root, 9, "forth");
        (root, _, _) = insert_single(root, 2, "fifth");

        print_as_binary_tree(&root);

        let node = find(&root, &4);

        root = splay(node);

        print_as_binary_tree(&root);

        let node = find(&root, &1);

        root = splay(node);

        print_as_binary_tree(&root);

        let node = find(&root, &9);

        root = splay(node);

        print_as_binary_tree(&root);
    }
}
