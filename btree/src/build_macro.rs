/// btreeを作成する
/// - `btree! {}`
#[macro_export]
macro_rules! btree {
    ( keys: $keys:expr , vals: $vals:expr , size: $size:expr $(,)* ) => {
        Some(Rc::new(RefCell::new(BTreeNode {
            parent: None,
            keys: $keys,
            vals: $vals,
            children: None,
            size: $size,
        })))
    };
    ( keys: $keys:expr , vals: $vals:expr , children: $children:expr , size: $size:expr $(,)* ) => {
        Some(Rc::new(RefCell::new(BTreeNode {
            parent: None,
            keys: $keys,
            vals: $vals,
            children: Some($children),
            size: $size,
        })))
    };
}
