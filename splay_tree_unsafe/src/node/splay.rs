use crate::node::{
    pointer::{NodeOps, NodePtr},
    state::NodeState,
};

/// nodeを1つ上に持ってくるように回転する
pub fn rotate<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    match node.get_state() {
        NodeState::Root => node,
        NodeState::LeftChild => {
            let mut right = node.take_right();
            let par = node.parent().clone();

            // 自分の右の子の親←親
            if let Some(right_inner) = right.as_mut() {
                *right_inner.parent_mut() = par.clone();
            }

            // 親はかならず存在する
            let mut par_inner = par.unwrap();

            // 親の左の子←自分の右の子
            *par_inner.left_mut() = right;

            // 自分の親←親の親
            let par_state = par_inner.get_state();
            let parpar = par_inner.parent_mut();

            if let Some(parpar_inner) = parpar.as_mut() {
                match par_state {
                    NodeState::LeftChild => {
                        *parpar_inner.left_mut() = Some(node.clone());
                    }
                    NodeState::RightChild => {
                        *parpar_inner.right_mut() = Some(node.clone());
                    }
                    _ => (),
                }
            }

            *node.parent_mut() = parpar.clone();

            // 自分の右の子←親
            *par_inner.parent_mut() = Some(node);
            node.right_mut().replace(par_inner);

            node
        }
        NodeState::RightChild => {
            let mut left = node.take_left();
            let par = node.parent().clone();

            // 自分の左の子の親←親
            if let Some(left_inner) = left.as_mut() {
                *left_inner.parent_mut() = par.clone();
            }

            // 親はかならず存在する
            let mut par_inner = par.unwrap();

            // 親の右の子←自分の左の子
            *par_inner.right_mut() = left;

            // 自分の親←親の親
            let par_state = par_inner.get_state();
            let parpar = par_inner.parent_mut();

            if let Some(parpar_inner) = parpar.as_mut() {
                match par_state {
                    NodeState::LeftChild => {
                        *parpar_inner.left_mut() = Some(node.clone());
                    }
                    NodeState::RightChild => {
                        *parpar_inner.right_mut() = Some(node.clone());
                    }
                    _ => (),
                }
            }

            *node.parent_mut() = parpar.clone();

            // 自分の左の子←親
            *par_inner.parent_mut() = Some(node);
            node.left_mut().replace(par_inner);

            node
        }
    }
}

/// スプレー操作によりnodeを根に移動し，新たな根を返す
pub fn splay<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
    while node.is_child() {
        // 頂点の状態
        let state = node.get_state();
        // 親頂点の状態（親は存在する）
        let par = node.parent().unwrap();
        let par_state = par.get_state();

        match (state, par_state) {
            (NodeState::Root, _) => {
                break;
            }
            // zig
            (NodeState::LeftChild | NodeState::RightChild, NodeState::Root) => {
                node = rotate(node);
            }
            // zig-zag
            (NodeState::LeftChild, NodeState::RightChild)
            | (NodeState::RightChild, NodeState::LeftChild) => {
                node = rotate(node);
                node = rotate(node);
            }
            // zig-zig
            (NodeState::LeftChild, NodeState::LeftChild)
            | (NodeState::RightChild, NodeState::RightChild) => {
                // 親を先にrotate（オブジェクトをdropさせないため，変数に代入する）
                let _par = rotate(node.parent().unwrap());
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
        node::{find::find, insert::insert, pointer::NodeOps, splay::rotate},
        utils::print::print_as_tree,
    };

    #[test]
    fn test_rotate_right() {
        let (node_1, node_3, node_5, mut node_15, mut node_30);
        let mut root = None;
        (root, node_5, _) = insert(root, 5, "first");
        (root, node_15, _) = insert(root, 15, "second");
        (root, node_1, _) = insert(root, 1, "third");
        (root, node_3, _) = insert(root, 3, "forth");
        (root, node_30, _) = insert(root, 30, "fifth");

        print_as_tree(&root);

        println!("find_5 = {:?}", node_5.get_state());

        // rootを回転
        println!("> rotate at root");
        root = Some(rotate(root.unwrap()));

        print_as_tree(&root);

        {
            println!("find_15 = {:?}", node_15.get_state());

            // 回転
            println!("> rotate 15");
            node_15 = rotate(node_15);

            println!("root = {:?}", root.as_ref().unwrap().get_state());
            println!("find_15 = {:?}", node_15.get_state());

            root = Some(node_15);

            print_as_tree(&root);
        }

        {
            println!("find_30 = {:?}", node_30.get_state());

            // 回転
            println!("> rotate 30");
            node_30 = rotate(node_30);

            println!("root = {:?}", root.as_ref().unwrap().get_state());
            println!("find_30 = {:?}", node_30.get_state());

            root = Some(node_30);

            print_as_tree(&root);
        }
    }

    #[test]
    fn test_rotate_left() {
        let mut root = None;
        (root, _, _) = insert(root, 5, "first");
        (root, _, _) = insert(root, 15, "second");
        (root, _, _) = insert(root, 1, "third");
        (root, _, _) = insert(root, 3, "forth");
        (root, _, _) = insert(root, 30, "fifth");

        print_as_tree(&root);

        {
            let mut find_30;
            (root, find_30) = find(root.clone(), &30);
            println!("find_30 = {:?}", find_30.as_ref().unwrap().get_state());

            // 回転
            println!("> rotate 30");
            find_30 = Some(rotate(find_30.unwrap()));

            print_as_tree(&root);
            print_as_tree(&find_30);

            println!("root = {:?}", root.as_ref().unwrap().get_state());
            println!("find_30 = {:?}", find_30.as_ref().unwrap().get_state());
        }

        {
            let mut find_30;
            (root, find_30) = find(root.clone(), &30);
            println!("find_30 = {:?}", find_30.as_ref().unwrap().get_state());

            // 回転
            println!("> rotate 30");
            find_30 = Some(rotate(find_30.unwrap()));

            print_as_tree(&root);
            print_as_tree(&find_30);

            println!("root = {:?}", root.as_ref().unwrap().get_state());
            println!("find_30 = {:?}", find_30.as_ref().unwrap().get_state());
        }
    }

    #[test]
    fn test_splay() {
        let mut root = None;

        (root, _, _) = insert(root, 1, "first");
        (root, _, _) = insert(root, 3, "second");
        (root, _, _) = insert(root, 4, "third");
        (root, _, _) = insert(root, 9, "forth");
        (root, _, _) = insert(root, 2, "fifth");

        print_as_tree(&root);

        (root, _) = find(root.clone(), &4);

        print_as_tree(&root);

        (root, _) = find(root.clone(), &1);

        print_as_tree(&root);

        (root, _) = find(root.clone(), &9);

        print_as_tree(&root);
    }
}
