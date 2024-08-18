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

/// 比較関数 cmp を満たす最小のノードを返す
fn find_min<K: Ord, V, F: Fn(&K) -> bool>(mut root: NodePtr<K, V>, cmp: F) -> NodePtr<K, V> {
    let mut res = None;

    while root.is_some() {
        if root.get_key().is_some_and(|k| cmp(&k)) {
            res = root.clone();
            root = root.get_left().map(|node| node.clone())?;
        } else {
            root = root.get_right().map(|node| node.clone())?;
        }
    }

    res
}

/// `x` 以上の値を持つ最小のノードを返す
pub fn lower_bound<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> NodePtr<K, V> {
    find_min(root, |k| k >= x)
}

/// `x` より大きい値を持つ最小のノードを返す
pub fn upper_bound<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> NodePtr<K, V> {
    find_min(root, |k| k > x)
}

#[cfg(test)]
mod test_find {
    use crate::{
        node::{find::find, insert::insert, node_pointer::NodeOps},
        print_util::print_as_binary_tree,
    };

    use super::{lower_bound, upper_bound};

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
    fn test_lowerbound_upperbound() {
        let mut root = None;
        (root, _) = insert(root, 5, "first");
        (root, _) = insert(root, 15, "second");
        (root, _) = insert(root, 1, "third");
        (root, _) = insert(root, 3, "forth");
        (root, _) = insert(root, 30, "fifth");

        print_as_binary_tree(&root);

        assert_eq!(*lower_bound(root.clone(), &0).get_key().unwrap(), 1);
        assert_eq!(*upper_bound(root.clone(), &0).get_key().unwrap(), 1);

        assert_eq!(*lower_bound(root.clone(), &1).get_key().unwrap(), 1);
        assert_eq!(*upper_bound(root.clone(), &1).get_key().unwrap(), 3);

        assert_eq!(*lower_bound(root.clone(), &5).get_key().unwrap(), 5);
        assert_eq!(*upper_bound(root.clone(), &5).get_key().unwrap(), 15);

        assert_eq!(*lower_bound(root.clone(), &10).get_key().unwrap(), 15);
        assert_eq!(*upper_bound(root.clone(), &10).get_key().unwrap(), 15);

        assert!(lower_bound(root.clone(), &100).get_key().is_none());
        assert!(upper_bound(root.clone(), &100).get_key().is_none());

        print_as_binary_tree(&root);
    }
}
