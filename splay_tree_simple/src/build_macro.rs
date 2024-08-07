#[macro_export]
macro_rules! tree_map {
    () => {
        None
    };
    ( key: $key:expr, value: $value:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            value: $value,
            left: None,
            right: None,
        }))
    };
    ( key: $key:expr, value: $value:expr, left: $left:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            value: $value,
            left: $left,
            right: None,
        }))
    };
    ( key: $key:expr, value: $value:expr, right: $right:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            value: $value,
            left: None,
            right: $right,
        }))
    };
    ( key: $key:expr, value: $value:expr, left: $left:expr, right: $right:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            value: $value,
            left: $left,
            right: $right,
        }))
    };
}

#[macro_export]
macro_rules! tree_multiset {
    () => {
        None
    };
    ( key: $key:expr, $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            id: 0,
            left: None,
            right: None,
        }))
    };
    ( key: $key:expr, left: $left:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            id: 0,
            left: $left,
            right: None,
        }))
    };
    ( key: $key:expr, right: $right:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            id: 0,
            left: None,
            right: $right,
        }))
    };
    ( key: $key:expr, left: $left:expr, right: $right:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            id: 0,
            left: $left,
            right: $right,
        }))
    };
}

#[macro_export]
macro_rules! tree_multiset_with_index {
    () => {
        None
    };
    ( key: $key:expr, size: $size:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            id: 0,
            size: $size,
            left: None,
            right: None,
        }))
    };
    ( key: $key:expr, size: $size:expr, left: $left:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            id: 0,
            size: $size,
            left: $left,
            right: None,
        }))
    };
    ( key: $key:expr, size: $size:expr, right: $right:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            id: 0,
            size: $size,
            left: None,
            right: $right,
        }))
    };
    ( key: $key:expr, size: $size:expr, left: $left:expr, right: $right:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            id: 0,
            size: $size,
            left: $left,
            right: $right,
        }))
    };
}
