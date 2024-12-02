//! 探索の実装

use crate::{node::NodePtr, node_util::NodeUtil};

/// 最も左の葉ノードを探索する
fn leftmost_leaf<const D: usize, K, V>(node: &NodePtr<D, K, V>) -> &NodePtr<D, K, V>
where
    [(); 2 * D - 1]:,
{
    let mut x = node;

    while let Some(left) = x.nth_child(0) {
        x = left;
    }

    x
}

/// 最も右の葉ノードを探索する
fn rightmost_leaf<const D: usize, K, V>(node: &NodePtr<D, K, V>) -> &NodePtr<D, K, V>
where
    [(); 2 * D - 1]:,
{
    let mut x = node;

    while let Some(right) = x.nth_child(*x.size()) {
        x = right;
    }

    x
}

/// 部分木の最小値を返す
pub fn min_key<const D: usize, K, V>(node: &NodePtr<D, K, V>) -> Option<&K>
where
    [(); 2 * D - 1]:,
{
    leftmost_leaf(node).keys[0].as_ref()
}

/// 部分木の最大値を返す
pub fn max_key<const D: usize, K, V>(node: &NodePtr<D, K, V>) -> Option<&K>
where
    [(); 2 * D - 1]:,
{
    let rightmost = rightmost_leaf(node);
    let size = *rightmost.size();
    rightmost.keys[size - 1].as_ref()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{btree, debug::print_as_tree, node::BTreeNode};

    #[test]
    fn test_leftmost_leaf() {
        let tree: Option<NodePtr<2, i32, &str>> = btree! {
            keys: [Some(1), Some(2), Some(3)],
            vals: [Some("a"), Some("b"), Some("c")],
            children: [
                btree! {
                    keys: [Some(0), None, None],
                    vals: [Some("x"), None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some(4), None, None],
                    vals: [Some("d"), None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some(5), None, None],
                    vals: [Some("e"), None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some(6), None, None],
                    vals: [Some("f"), None, None],
                    size: 1,
                },
            ],
            size: 3,
        };

        print_as_tree(&tree);

        let leftmost = leftmost_leaf(&tree.as_ref().unwrap());
        let rightmost = rightmost_leaf(&tree.as_ref().unwrap());

        print!("leftmost: {:?}", leftmost);
        print!("rightmost: {:?}", rightmost);

        assert_eq!(*leftmost.nth_key(0).unwrap(), 0);
        assert_eq!(*rightmost.nth_key(0).unwrap(), 6);
    }

    #[test]
    fn test_min_max() {
        let tree: Option<NodePtr<2, i32, &str>> = btree! {
            keys: [Some(1), Some(2), Some(3)],
            vals: [Some("a"), Some("b"), Some("c")],
            children: [
                btree! {
                    keys: [Some(0), None, None],
                    vals: [Some("x"), None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some(4), None, None],
                    vals: [Some("d"), None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some(5), None, None],
                    vals: [Some("e"), None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some(6), None, None],
                    vals: [Some("f"), None, None],
                    size: 1,
                },
            ],
            size: 3,
        };

        print_as_tree(&tree);

        let min = min_key(&tree.as_ref().unwrap());
        let max = max_key(&tree.as_ref().unwrap());

        println!("min: {:?}", min);
        println!("max: {:?}", max);

        assert_eq!(*min.unwrap(), 0);
        assert_eq!(*max.unwrap(), 6);
    }
}
