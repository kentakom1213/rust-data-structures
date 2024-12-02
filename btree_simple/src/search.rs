//! 探索の実装

use crate::{node::NodePtr, node_util::NodeUtil};

/// 最も左の葉ノードを探索する
pub fn leftmost_leaf<const D: usize, K, V>(
    x: &Option<NodePtr<D, K, V>>,
) -> Option<&NodePtr<D, K, V>>
where
    [(); 2 * D - 1]:,
{
    let mut x = x.as_ref()?;

    while !x.is_leaf() {
        x = x.nth_child(0)?;
    }

    Some(x)
}

/// 最も右の葉ノードを探索する
pub fn rightmost_leaf<const D: usize, K, V>(
    x: &Option<NodePtr<D, K, V>>,
) -> Option<&NodePtr<D, K, V>>
where
    [(); 2 * D - 1]:,
{
    let mut x = x.as_ref()?;

    while !x.is_leaf() {
        let size = *x.size();
        x = x.nth_child(size)?;
    }

    Some(x)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{btree, debug::print_as_tree, node::BTreeNode, node_util::NodeUtil};

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

        let leftmost = leftmost_leaf(&tree);
        let rightmost = rightmost_leaf(&tree);

        print!("leftmost: {:?}", leftmost);
        print!("rightmost: {:?}", rightmost);

        assert_eq!(*leftmost.unwrap().nth_key(0).unwrap(), 0);
        assert_eq!(*rightmost.unwrap().nth_key(0).unwrap(), 6);
    }
}
