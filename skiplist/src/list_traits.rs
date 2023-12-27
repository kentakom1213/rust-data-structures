//! リストのトレイト

pub trait LinkedList<T>
where
    T: Eq,
{
    /// 空のリストを作成する
    fn new() -> Self;
}
