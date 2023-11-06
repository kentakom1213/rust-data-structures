#[macro_export]
macro_rules! tree {
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
