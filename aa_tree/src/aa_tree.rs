#![allow(non_snake_case)]

/// AA木のノード
#[derive(Debug)]
pub struct AATreeNodeInner<K, V>
where
    K: Ord,
{
    pub key: K,
    pub value: V,
    pub level: usize,
    pub left: Option<Box<AATreeNodeInner<K, V>>>,
    pub right: Option<Box<AATreeNodeInner<K, V>>>,
}

pub type AATreeNode<K, V> = Option<Box<AATreeNodeInner<K, V>>>;

/// ノードの逆転
/// ```text
///   |        ⇓           ⇓        
/// 2 |    L ← T           L → T    
///   |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
/// 1 |  A   B   R       A   B   R  
/// ```
fn skew<K: Ord, V>(node: AATreeNode<K, V>) -> AATreeNode<K, V> {
    let Some(mut T) = node else {
        return None;
    };
    if T.left.is_none() {
        Some(T)
    } else if T.level == T.left.as_ref().unwrap().level {
        // ポインタの入れ替え
        let mut L = T.left.unwrap();
        T.left = L.right;
        L.right = Some(T);
        Some(L)
    } else {
        Some(T)
    }
}

/// ノードの分割操作
/// ```text
///   |                         ⇓    
/// 3 |                         R    
///   |    ⇓                   ↙ ↘   
/// 2 |    T → R → X   ==>    T   X  
///   |   ↙   ↙              ↙ ↘     
/// 1 |  A   B              A   B    
/// ```
fn split<K: Ord, V>(node: AATreeNode<K, V>) -> AATreeNode<K, V> {
    let Some(mut T) = node else {
        return None;
    };
    if T.right.is_none() || T.right.as_ref().unwrap().right.is_none() {
        Some(T)
    } else if T.level == T.right.as_ref().unwrap().right.as_ref().unwrap().level {
        let mut R = T.right.unwrap();
        T.right = R.left;
        R.left = Some(T);
        R.level += 1; // Rのレベルを1上げる
        Some(R)
    } else {
        Some(T)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    fn test_skew() {
        let mut tree = tree! {
            key: "D",
            value: 4,
            level: 2,
            left: tree! {
                key: "B",
                value: 2,
                level: 2,
                left: tree! {
                    key: "A",
                    value: 1,
                    level: 1,
                },
                right: tree! {
                    key: "C",
                    value: 3,
                    level: 1,
                },
            },
            right: tree! {
                key: "E",
                value: 5,
                level: 1,
            }
        };

        println!("----- before skew -----");
        println!("{:#?}", &tree);

        // skew
        tree = skew(tree);

        println!("----- after skew -----");
        println!("{:#?}", &tree);
    }

    #[test]
    fn test_split() {
        let mut tree = tree! {
            key: "B",
            value: 2,
            level: 2,
            left: tree! {
                key: "A",
                value: 1,
                level: 1,
            },
            right: tree! {
                key: "D",
                value: 4,
                level: 2,
                left: tree! {
                    key: "C",
                    value: 3,
                    level: 1,
                },
                right: tree! {
                    key: "E",
                    value: 5,
                    level: 2,
                }
            }
        };

        println!("----- before split -----");
        println!("{:#?}", &tree);

        // split
        tree = split(tree);

        println!("----- after split -----");
        println!("{:#?}", &tree);
    }
}
