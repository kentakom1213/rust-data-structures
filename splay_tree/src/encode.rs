use crate::map::*;
use std::fmt::Debug;

pub trait Encode {
    fn encode(&self) -> String;
}

impl<T: Ord + Debug, U> Encode for SplayTreeMap<T, U> {
    /// ## encode
    /// かっこ区切りの文字列にフォーマットする
    fn encode(&self) -> String {
        let mut res = String::new();
        encode_inner(&self.root, &mut res);
        res
    }
}

/// 木をかっこ区切りの文字列に変換する
fn encode_inner<T: Ord + Debug, U>(root: &Option<Box<Node<T, U>>>, res: &mut String) {
    if let Some(root) = root {
        let res_l = if root.left.is_some() {
            let mut res_l = String::new();
            encode_inner(&root.left, &mut res_l);
            res_l
        } else {
            String::new()
        };
        let res_r = if root.right.is_some() {
            let mut res_r = String::new();
            encode_inner(&root.right, &mut res_r);
            res_r
        } else {
            String::new()
        };
        *res = format!("({}{:?}{})", res_l, &root.key, res_r);
    }
}
