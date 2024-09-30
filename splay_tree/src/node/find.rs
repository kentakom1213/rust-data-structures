use super::{
    iterator::NodePosition,
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
    let mut last = root.clone();
    let mut res = None;

    while last.is_some() {
        if last.key().is_some_and(|k| cmp(&k)) {
            res = last.clone();
            last = match last.left().map(|node| node.clone()) {
                Some(node) => node,
                None => break,
            };
        } else {
            last = match last.right().map(|node| node.clone()) {
                Some(node) => node,
                None => break,
            };
        }
    }

    if res.is_some() {
        (splay(res.clone()), res)
    } else if last.is_some() {
        (splay(last), res)
    } else {
        (splay(root), res)
    }
}

/// `x` 以上の値を持つ最小のノードを返す
///
/// **戻り値**
/// - `NodePtr<K, V>`: 検索後の根ノード
/// - `NodePtr<K, V>`: `x` 以上の値を持つ最小のノード
pub fn lower_bound<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePosition<K, V>) {
    let (new_root, node) = find_min(root, |k| k >= x);

    if node.is_some() {
        (new_root, NodePosition::Node(node))
    } else {
        (new_root, NodePosition::SUP)
    }
}

/// `x` より大きい値を持つ最小のノードを返す
///
/// **戻り値**
/// - `NodePtr<K, V>`: 検索後の根ノード
/// - `NodePtr<K, V>`: `x` より大きい値を持つ最小のノード
pub fn upper_bound<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePosition<K, V>) {
    let (new_root, node) = find_min(root, |k| k > x);

    if node.is_some() {
        (new_root, NodePosition::Node(node))
    } else {
        (new_root, NodePosition::SUP)
    }
}

/// 値 `x` を持つノードを返す
///
/// **戻り値**
/// - `NodePtr<K, V>`: 検索後の根ノード
/// - `NodePtr<K, V>`: 値 `x` を持つノード
pub fn find<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePtr<K, V>) {
    let (new_root, lb) = find_min(root.clone(), |k| k >= x);
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

        let mut found;
        let mut found_node;

        // 0
        (root, found) = lower_bound(root, &0);
        assert_eq!(*found.as_ref().unwrap().key().unwrap(), 1);
        print_as_tree(&root);

        (root, found) = upper_bound(root, &0);
        assert_eq!(*found.as_ref().unwrap().key().unwrap(), 1);
        print_as_tree(&root);

        (root, found_node) = find(root, &0);
        assert!(found_node.key().is_none());
        print_as_tree(&root);

        // 1
        (root, found) = lower_bound(root, &1);
        assert_eq!(*found.as_ref().unwrap().key().unwrap(), 1);
        print_as_tree(&root);

        (root, found) = upper_bound(root, &1);
        assert_eq!(*found.as_ref().unwrap().key().unwrap(), 3);
        print_as_tree(&root);

        (root, found_node) = find(root, &1);
        assert_eq!(*found_node.key().unwrap(), 1);
        print_as_tree(&root);

        // 5
        (root, found) = lower_bound(root, &5);
        assert_eq!(*found.as_ref().unwrap().key().unwrap(), 5);
        print_as_tree(&root);

        (root, found) = upper_bound(root, &5);
        assert_eq!(*found.as_ref().unwrap().key().unwrap(), 15);
        print_as_tree(&root);

        (root, found_node) = find(root, &5);
        assert_eq!(*found_node.key().unwrap(), 5);
        print_as_tree(&root);

        // 10
        (root, found) = lower_bound(root, &10);
        assert_eq!(*found.as_ref().unwrap().key().unwrap(), 15);
        print_as_tree(&root);

        (root, found) = upper_bound(root, &10);
        assert_eq!(*found.as_ref().unwrap().key().unwrap(), 15);
        print_as_tree(&root);

        (root, found_node) = find(root, &10);
        assert!(found_node.key().is_none());
        print_as_tree(&root);

        // 100
        (root, found) = lower_bound(root, &100);
        assert!(found.as_ref().is_none());
        print_as_tree(&root);

        (root, found) = upper_bound(root, &100);
        assert!(found.as_ref().is_none());
        print_as_tree(&root);

        (root, found_node) = find(root, &100);
        assert!(found_node.key().is_none());
        print_as_tree(&root);
    }
}
