/// ## Monoid
pub trait Monoid {
    /// ### DataType
    /// モノイドのデータ型
    type E: Clone;
    
    /// ### IdentityElement
    /// 単位元
    const I: Self::E;

    /// ### BinaryOperation
    /// 二項演算
    fn op(x: &Self::E, y: &Self::E) -> Self::E;
}

/// ## Group
pub trait Group : Monoid {
    /// ### Inverse
    /// 逆元を求める関数
    fn inv(x: &Self::E) -> Self::E;
}
