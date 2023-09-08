#[macro_export]
macro_rules! tree {
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
