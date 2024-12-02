/// btreeを作成する
/// - `btree! {}`
#[macro_export]
macro_rules! btree {
    ( keys: $keys:expr , vals: $vals:expr , size: $size:expr $(,)* ) => {
        Some(Box::new(BTreeNode {
            keys: $keys,
            vals: $vals,
            children: None,
            size: $size,
        }))
    };
    ( keys: $keys:expr , vals: $vals:expr , children: $children:expr , size: $size:expr $(,)* ) => {
        Some(Box::new(BTreeNode {
            keys: $keys,
            vals: $vals,
            children: Some($children),
            size: $size,
        }))
    };
}
