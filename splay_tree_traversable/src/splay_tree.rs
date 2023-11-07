//! 親へのポインタも持つスプレー木

/// スプレー木のノード
pub struct Node<T, U> {
    pub key: T,
    pub value: U,
    parent: Option<*mut Node<T, U>>,
    left: Option<*mut Node<T, U>>,
    right: Option<*mut Node<T, U>>,
}

impl<T, U> Node<T, U> {
    pub fn new(key: T, value: U) -> Self {
        Self {
            key,
            value,
            parent: None,
            left: None,
            right: None,
        }
    }
}

/// スプレー木
pub struct SplayTree<T, U> {
    size: usize,
    pub root: Option<*mut Node<T, U>>,
}

impl<T, U> SplayTree<T, U> {
    /// 木の初期化
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
