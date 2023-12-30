//! 木を簡易に作成するためのマクロ

#[macro_export]
macro_rules! tree {
    (key: $key:expr, value: $value:expr, level: $level:expr $(,)*) => {
        Some(Rc::new(RefCell::new(AATreeNodeInner {
            key: $key,
            value: $value,
            level: $level,
            left: None,
            right: None,
        })))
    };
    (key: $key:expr, value: $value:expr, level: $level:expr, left: $left:expr $(,)*) => {
        Some(Rc::new(RefCell::new(AATreeNodeInner {
            key: $key,
            value: $value,
            level: $level,
            left: $left,
            right: None,
        })))
    };
    (key: $key:expr, value: $value:expr, level: $level:expr, right: $right:expr $(,)*) => {
        Some(Rc::new(RefCell::new(AATreeNodeInner {
            key: $key,
            value: $value,
            level: $level,
            left: None,
            right: $right,
        })))
    };
    (key: $key:expr, value: $value:expr, level: $level:expr, left: $left:expr, right: $right:expr $(,)*) => {
        Some(Rc::new(RefCell::new(AATreeNodeInner {
            key: $key,
            value: $value,
            level: $level,
            left: $left,
            right: $right,
        })))
    };
}
