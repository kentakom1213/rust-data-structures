use super::{
    pointer::{NodeOps, NodePtr},
    splay::splay,
};

/// 比較関数 cmp を満たす最小のノードを返す
///
/// **戻り値**
/// - `NodePtr<K, V>`: 検索後の根ノード
/// - `NodePtr<K, V>`: 比較関数 cmp を満たす最小のノード
fn find_min<K: Ord, V, F: Fn(&K) -> bool>(
    root: NodePtr<K, V>,
    cmp: F,
) -> (NodePtr<K, V>, NodePtr<K, V>) {
    let mut root = root.clone();
    let mut res = None;

    while root.is_some() {
        if root.key().is_some_and(|k| cmp(&k)) {
            res = root.clone();
            root = match root.left().map(|node| node.clone()) {
                Some(node) => node,
                None => return (splay(res), None),
            };
        } else {
            root = match root.right().map(|node| node.clone()) {
                Some(node) => node,
                None => {
                    if res.is_some() {
                        return (splay(res), None);
                    } else {
                        return (splay(root), None);
                    }
                }
            };
        }
    }

    (splay(res.clone()), res)
}

/// `x` 以上の値を持つ最小のノードを返す
///
/// **戻り値**
/// - `NodePtr<K, V>`: 検索後の根ノード
/// - `NodePtr<K, V>`: `x` 以上の値を持つ最小のノード
pub fn lower_bound<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePtr<K, V>) {
    find_min(root, |k| k >= x)
}

/// `x` より大きい値を持つ最小のノードを返す
///
/// **戻り値**
/// - `NodePtr<K, V>`: 検索後の根ノード
/// - `NodePtr<K, V>`: `x` より大きい値を持つ最小のノード
pub fn upper_bound<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePtr<K, V>) {
    find_min(root, |k| k > x)
}

/// 値 `x` を持つノードを返す
///
/// **戻り値**
/// - `NodePtr<K, V>`: 検索後の根ノード
/// - `NodePtr<K, V>`: 値 `x` を持つノード
pub fn find<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePtr<K, V>) {
    let (new_root, lb) = lower_bound(root.clone(), x);
    if lb.key().is_some_and(|k| &*k == x) {
        (new_root, lb)
    } else {
        (new_root, None)
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

        // 0
        (root, _) = lower_bound(root, &0);
        assert_eq!(*root.key().unwrap(), 1);
        print_as_tree(&root);

        (root, _) = upper_bound(root, &0);
        assert_eq!(*root.key().unwrap(), 1);
        print_as_tree(&root);

        (root, _) = find(root, &0);
        print_as_tree(&root);

        // 1
        (root, _) = lower_bound(root, &1);
        assert_eq!(*root.key().unwrap(), 1);
        print_as_tree(&root);

        (root, _) = upper_bound(root, &1);
        assert_eq!(*root.key().unwrap(), 3);
        print_as_tree(&root);

        (root, _) = find(root, &1);
        assert_eq!(*root.key().unwrap(), 1);
        print_as_tree(&root);

        // 5
        (root, _) = lower_bound(root, &5);
        assert_eq!(*root.key().unwrap(), 5);
        print_as_tree(&root);

        (root, _) = upper_bound(root, &5);
        assert_eq!(*root.key().unwrap(), 15);
        print_as_tree(&root);

        (root, _) = find(root, &5);
        assert_eq!(*root.key().unwrap(), 5);
        print_as_tree(&root);

        // 10
        (root, _) = lower_bound(root, &10);
        assert_eq!(*root.key().unwrap(), 15);
        print_as_tree(&root);

        (root, _) = upper_bound(root, &10);
        assert_eq!(*root.key().unwrap(), 15);
        print_as_tree(&root);

        (root, _) = find(root, &10);
        assert!(root.key().is_none());
        print_as_tree(&root);

        // 100
        (root, _) = lower_bound(root, &100);
        assert!(root.key().is_none());
        print_as_tree(&root);

        (root, _) = upper_bound(root, &100);
        assert!(root.key().is_none());
        print_as_tree(&root);

        (root, _) = find(root, &100);
        assert!(root.key().is_none());
        print_as_tree(&root);
    }
}
