/// # Node
pub struct Node<T, U>
where
    T: Ord,
{
    pub key: T,
    pub val: U,
    pub left: Box<Option<Node<T, U>>>,
    pub right: Box<Option<Node<T, U>>>,
}

/// # SplayTree
/// スプレー木のクラス
pub struct SplayTree<T, U>
where
    T: Ord,
{
    size: usize,
    root: Option<Node<T, U>>,
}

impl<T, U> SplayTree<T, U>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }
}
