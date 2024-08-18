use super::{node_pointer::NodeOps, NodePtr};

/// 比較関数 cmp を満たす最小のノードを返す
fn find_min<K: Ord, V, F: Fn(&K) -> bool>(mut root: NodePtr<K, V>, cmp: F) -> NodePtr<K, V> {
    let mut res = None;

    while root.is_some() {
        if root.key().is_some_and(|k| cmp(&k)) {
            res = root.clone();
            root = root.left().map(|node| node.clone())?;
        } else {
            root = root.right().map(|node| node.clone())?;
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

/// 値 `x` を持つノードを返す
pub fn find<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> NodePtr<K, V> {
    let lb = lower_bound(root, x);
    if &*lb.key()? == x {
        lb
    } else {
        None
    }
}

#[cfg(test)]
mod test_find {
    use crate::{
        node::{find::find, insert::insert, node_pointer::NodeOps},
        print_util::print_as_binary_tree,
    };

    use super::{lower_bound, upper_bound};

    #[test]
    fn test_find_lowerbound_upperbound() {
        let mut root = None;
        (root, _) = insert(root, 5, "first");
        (root, _) = insert(root, 15, "second");
        (root, _) = insert(root, 1, "third");
        (root, _) = insert(root, 3, "forth");
        (root, _) = insert(root, 30, "fifth");

        print_as_binary_tree(&root);

        assert_eq!(*lower_bound(root.clone(), &0).key().unwrap(), 1);
        assert_eq!(*upper_bound(root.clone(), &0).key().unwrap(), 1);
        assert!(find(root.clone(), &0).key().is_none());

        assert_eq!(*lower_bound(root.clone(), &1).key().unwrap(), 1);
        assert_eq!(*upper_bound(root.clone(), &1).key().unwrap(), 3);
        assert_eq!(*find(root.clone(), &1).key().unwrap(), 1);

        assert_eq!(*lower_bound(root.clone(), &5).key().unwrap(), 5);
        assert_eq!(*upper_bound(root.clone(), &5).key().unwrap(), 15);
        assert_eq!(*find(root.clone(), &5).key().unwrap(), 5);

        assert_eq!(*lower_bound(root.clone(), &10).key().unwrap(), 15);
        assert_eq!(*upper_bound(root.clone(), &10).key().unwrap(), 15);
        assert!(find(root.clone(), &10).key().is_none());

        assert!(lower_bound(root.clone(), &100).key().is_none());
        assert!(upper_bound(root.clone(), &100).key().is_none());
        assert!(find(root.clone(), &100).key().is_none());

        print_as_binary_tree(&root);
    }
}
