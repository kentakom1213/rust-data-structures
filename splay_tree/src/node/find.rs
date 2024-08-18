use std::cmp::Ordering;

use super::{node_pointer::NodeOps, NodePtr};

/// rootを根とする木で，xに一致するキーをもつノードの参照を返す
pub fn find<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePtr<K, V>) {
    let mut node = root.clone();
    while node.is_some() {
        let comp = x.cmp(&node.get_key().as_ref().unwrap());
        node = match comp {
            Ordering::Less => node.get_left().unwrap().clone(),
            Ordering::Equal => break,
            Ordering::Greater => node.get_right().unwrap().clone(),
        }
    }
    (root, node)
}

#[cfg(test)]
mod test_find {
    use crate::{
        node::{find::find, insert::insert},
        print_util::print_as_binary_tree,
    };

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
}
