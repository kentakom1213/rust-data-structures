#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ( key: $key:expr, $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            left: None,
            right: None,
        }))
    };
    ( key: $key:expr, left: $left:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            left: $left,
            right: None,
        }))
    };
    ( key: $key:expr, right: $right:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            left: None,
            right: $right,
        }))
    };
    ( key: $key:expr, left: $left:expr, right: $right:expr $(,)* ) => {
        Some(Box::new(Node {
            key: $key,
            left: $left,
            right: $right,
        }))
    };
}
