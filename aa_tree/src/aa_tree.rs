#![allow(non_snake_case)]

/// AA木のノード
pub struct AATreeNodeInner<K, V>
where
    K: Ord,
{
    key: K,
    value: V,
    level: usize,
    left: Option<Box<AATreeNodeInner<K, V>>>,
    right: Option<Box<AATreeNodeInner<K, V>>>,
}

pub type AATreeNode<K, V> = Option<Box<AATreeNodeInner<K, V>>>;

/// ノードの逆転
/// ```text
///       ⇓           ⇓       
///   L ← T           L → T   
///  ↙ ↘   ↘   ==>   ↙   ↙ ↘  
/// A   B   R       A   B   R
/// ```
fn skew<K: Ord, V>(mut node: AATreeNode<K, V>) -> AATreeNode<K, V> {
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
///                       ⇓    
///   ⇓                   R    
///   T → R → X          ↙ ↘   
///  ↙   ↙       ==>    T   X  
/// A   B              ↙ ↘     
///                   A   B    
/// ```
fn split<K: Ord, V>(node: AATreeNode<K, V>) -> AATreeNode<K, V> {
    todo!()
}
