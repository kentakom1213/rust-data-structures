//! 代数的構造

use std::fmt::Debug;

/// モノイド
pub trait Monoid {
    /// 元の型
    type Val: Debug + Clone + PartialEq;
    /// 単位元
    const E: Self::Val;
    /// 演算
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

pub mod monoids {
    use super::Monoid;

    /// 和
    #[derive(Debug, Clone)]
    pub struct Add;
    impl Monoid for Add {
        type Val = isize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }

    /// 文字列
    #[derive(Debug, Clone)]
    pub struct Str;
    impl Monoid for Str {
        type Val = String;
        const E: Self::Val = String::new();
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left.to_string() + right
        }
    }
}
