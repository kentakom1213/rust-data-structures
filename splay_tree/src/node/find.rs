use super::pointer::{NodeOps, NodePtr};

/// 比較関数 cmp を満たす最小のノードを返す
fn find_min<K: Ord, V, F: Fn(&K) -> bool>(root: &NodePtr<K, V>, cmp: F) -> NodePtr<K, V> {
    let mut root = root.clone();
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
pub fn lower_bound<K: Ord, V>(root: &NodePtr<K, V>, x: &K) -> NodePtr<K, V> {
    find_min(root, |k| k >= x)
}

/// `x` より大きい値を持つ最小のノードを返す
pub fn upper_bound<K: Ord, V>(root: &NodePtr<K, V>, x: &K) -> NodePtr<K, V> {
    find_min(root, |k| k > x)
}

/// 値 `x` を持つノードを返す
pub fn find<K: Ord, V>(root: &NodePtr<K, V>, x: &K) -> NodePtr<K, V> {
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
        node::{find::find, insert::insert, pointer::NodeOps},
        print_util::print_as_tree,
    };

    use super::{lower_bound, upper_bound};

    #[test]
    fn test_find_lowerbound_upperbound() {
        let mut root = None;
        (root, _, _) = insert(root, 5, "first");
        (root, _, _) = insert(root, 15, "second");
        (root, _, _) = insert(root, 1, "third");
        (root, _, _) = insert(root, 3, "forth");
        (root, _, _) = insert(root, 30, "fifth");

        print_as_tree(&root);

        assert_eq!(*lower_bound(&root, &0).key().unwrap(), 1);
        assert_eq!(*upper_bound(&root, &0).key().unwrap(), 1);
        assert!(find(&root, &0).key().is_none());

        assert_eq!(*lower_bound(&root, &1).key().unwrap(), 1);
        assert_eq!(*upper_bound(&root, &1).key().unwrap(), 3);
        assert_eq!(*find(&root, &1).key().unwrap(), 1);

        assert_eq!(*lower_bound(&root, &5).key().unwrap(), 5);
        assert_eq!(*upper_bound(&root, &5).key().unwrap(), 15);
        assert_eq!(*find(&root, &5).key().unwrap(), 5);

        assert_eq!(*lower_bound(&root, &10).key().unwrap(), 15);
        assert_eq!(*upper_bound(&root, &10).key().unwrap(), 15);
        assert!(find(&root, &10).key().is_none());

        assert!(lower_bound(&root, &100).key().is_none());
        assert!(upper_bound(&root, &100).key().is_none());
        assert!(find(&root, &100).key().is_none());

        print_as_tree(&root);
    }
}
