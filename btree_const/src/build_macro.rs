/// btreeを作成する
/// - `btree! {}`
#[macro_export]
macro_rules! btree {
    ( keys: $keys:expr , vals: $vals:expr , size: $size:expr $(,)* ) => {
        Some(Rc::new(RefCell::new(Node::Leaf {
            parent: None,
            keys: $keys,
            vals: $vals,
            size: $size,
        })))
    };
    ( keys: $keys:expr , vals: $vals:expr , children: $children:expr , size: $size:expr $(,)* ) => {
        Some(Rc::new(RefCell::new(Node::Internal {
            parent: None,
            keys: $keys,
            vals: $vals,
            children: $children,
            size: $size,
        })))
    };
}
