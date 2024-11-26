//! ノードのユーティリティ

use std::cell::{Ref, RefMut};

use crate::node::NodePtr;

pub trait NodeUtil<const D: usize, K, V>
where
    [(); 2 * D - 1]:,
{
    /// ノードの要素数を取得する
    fn size(&self) -> Ref<usize>;
    /// ノードの要素数への可変参照を取得する
    fn size_mut(&mut self) -> RefMut<usize>;
    /// キーの配列への参照を取得する
    fn keys(&self) -> Ref<[Option<K>; 2 * D - 1]>;
    /// キーの配列への可変参照を取得する
    fn keys_mut(&mut self) -> RefMut<[Option<K>; 2 * D - 1]>;
    /// 値の配列への不変参照を取得する
    fn vals(&self) -> Ref<[Option<V>; 2 * D - 1]>;
    /// 値の配列への可変参照を取得する
    fn vals_mut(&mut self) -> RefMut<[Option<V>; 2 * D - 1]>;
    /// 子ノードの配列への不変参照を取得する
    fn children(&self) -> Ref<Option<[Option<NodePtr<D, K, V>>; 2 * D]>>;
    /// 子ノードの配列への可変参照を取得する
    fn children_mut(&mut self) -> RefMut<Option<[Option<NodePtr<D, K, V>>; 2 * D]>>;
    /// 空きが存在するか判定
    fn is_filled(&self) -> bool {
        *self.size() == 2 * D - 1
    }
}

macro_rules! impl_get_ref {
    ($name:ident, $field:ident, $return:ty) => {
        fn $name(&self) -> $return {
            Ref::map(self.borrow(), |p| &p.$field)
        }
    };
    ($name:ident, $field:ident, $return:ty, mut) => {
        fn $name(&mut self) -> $return {
            RefMut::map(self.borrow_mut(), |p| &mut p.$field)
        }
    };
}

impl<const D: usize, K, V> NodeUtil<D, K, V> for NodePtr<D, K, V>
where
    [(); 2 * D - 1]:,
    [(); 2 * D]:,
{
    impl_get_ref!(size, size, Ref<usize>);
    impl_get_ref!(size_mut, size, RefMut<usize>, mut);
    impl_get_ref!(keys, keys, Ref<[Option<K>; 2 * D - 1]>);
    impl_get_ref!(keys_mut, keys, RefMut<[Option<K>; 2 * D - 1]>, mut);
    impl_get_ref!(vals, vals, Ref<[Option<V>; 2 * D - 1]>);
    impl_get_ref!(vals_mut, vals, RefMut<[Option<V>; 2 * D - 1]>, mut);
    impl_get_ref!(
        children,
        children,
        Ref<Option<[Option<NodePtr<D, K, V>>; 2 * D]>>
    );
    impl_get_ref!(
        children_mut,
        children,
        RefMut<Option<[Option<NodePtr<D, K, V>>; 2 * D]>>,
        mut
    );
}
