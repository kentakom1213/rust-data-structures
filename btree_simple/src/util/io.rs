//! jsonとB木の相互変換

use serde::{ser::SerializeStruct, Serialize};

use crate::BTreeNode;

impl<const D: usize, K, V> Serialize for BTreeNode<D, K, V>
where
    [(); 2 * D - 1]:,
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let BTreeNode {
            keys,
            vals,
            size,
            children,
        } = &self;

        let keys_vec = (0..*size)
            .map(|i| keys[i].as_ref().unwrap())
            .collect::<Vec<_>>();
        let vals_vec = (0..*size)
            .map(|i| vals[i].as_ref().unwrap())
            .collect::<Vec<_>>();

        let mut state =
            serializer.serialize_struct("BTreeNode", if children.is_some() { 3 } else { 2 })?;

        state.serialize_field("keys", &keys_vec)?;
        state.serialize_field("vals", &vals_vec)?;

        if let Some(children) = children {
            let children_vec = (0..*size + 1)
                .map(|i| children[i].as_ref().unwrap())
                .collect::<Vec<_>>();
            state.serialize_field("children", &children_vec)?;
        }

        state.end()
    }
}

#[cfg(test)]
mod test {
    use crate::{btree, BTreeNode, NodePtr};

    #[test]
    fn test_serialize_node_1() {
        let tree: Option<NodePtr<3, char, &str>> = btree! {
            keys: [Some('b'), Some('e'), Some('g'), None, None],
            vals: [Some("Bob"), Some("Emily"), Some("Grace"), None, None],
            children: [
                btree! {
                    keys: [Some('a'), None, None, None, None],
                    vals: [Some("Alice"), None, None, None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some('c'), Some('d'), None, None, None],
                    vals: [Some("Charlie"), Some("David"), None, None, None],
                    size: 2,
                },
                btree! {
                    keys: [Some('f'), None, None, None, None],
                    vals: [Some("Frank"), None, None, None, None],
                    size: 1,
                },
                btree! {
                    keys: [Some('h'), None, None, None, None],
                    vals: [Some("Helen"), None, None, None, None],
                    size: 1,
                },
                None,
                None,
            ],
            size: 3
        };

        let serialized = serde_json::to_string_pretty(&tree).unwrap();

        println!("{}", serialized);
    }

    #[test]
    fn test_serialize_node_2() {
        let tree: Option<NodePtr<2, u32, ()>> = btree! {
            keys: [Some(11), None, None],
            vals: [Some(()), None, None],
            children: [
                btree! {
                    keys: [Some(8), None, None],
                    vals: [Some(()), None, None],
                    children: [
                        btree! {
                            keys: [Some(4), Some(7), None],
                            vals: [Some(()), Some(()), None],
                            size: 2
                        },
                        btree! {
                            keys: [Some(9), Some(10), None],
                            vals: [Some(()), Some(()), None],
                            size: 2
                        },
                        None,
                        None,
                    ],
                    size: 1
                },
                btree! {
                    keys: [Some(18), None, None],
                    vals: [Some(()), None, None],
                    children: [
                        btree! {
                            keys: [Some(14), Some(15), None],
                            vals: [Some(()), Some(()), None],
                            size: 2
                        },
                        btree! {
                            keys: [Some(18), Some(20), Some(20)],
                            vals: [Some(()), Some(()), Some(())],
                            size: 3
                        },
                        None,
                        None,
                    ],
                    size: 1
                },
                None,
                None,
            ],
            size: 1,
        };

        let serialized = serde_json::to_string_pretty(&tree).unwrap();

        println!("{}", serialized);
    }
}
