/// AA木のノード
pub struct Node<K, V>
where
    K: Ord,
{
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}


