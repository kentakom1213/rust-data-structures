//! 手動作成用マクロ

// use std::{
//     cell::RefCell,
//     rc::{Rc, Weak},
// };

// /// btreeを作成する
// /// ```
// /// let tree = btree! {};
// /// ```
// #[macro_export]
// macro_rules! btree {
//     ( keys: $keys:expr , vals: $vals:expr , len: $len:expr $(,)* ) => {
//         Some(Rc::new(RefCell::new(Node::Leaf {
//             parent: None,
//             keys: $keys,
//             vals: $vals,
//             len: $len,
//         })))
//     };
//     ( keys: $keys:expr , vals: $vals:expr , children: $children:expr , len: $len:expr $(,)* ) => {
//         Some(Rc::new(RefCell::new(Node::Internal {
//             parent: None,
//             keys: $keys,
//             vals: $vals,
//             children: $children,
//             len: $len,
//         })))
//     };
// }
