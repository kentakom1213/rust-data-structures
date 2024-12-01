//! ノードのユーティリティ

use std::cell::{Ref, RefMut};

use crate::node::{NodePtr, ParentPtr};

pub trait NodeUtil<const D: usize, K, V>
where
    [(); 2 * D - 1]:,
{
    /// ノードの要素数を取得する
    fn size(&self) -> Ref<usize>;
    /// ノードの要素数への可変参照を取得する
    fn size_mut(&mut self) -> RefMut<usize>;
    /// キーの配列への不変参照を取得する
    fn keys(&self) -> Ref<[Option<K>; 2 * D - 1]>;
    /// キーの配列への可変参照を取得する
    fn keys_mut(&mut self) -> RefMut<[Option<K>; 2 * D - 1]>;
    /// n番目のキーへの不変参照を取得する
    fn nth_key(&self, n: usize) -> Option<Ref<K>>;
    /// 値の配列への不変参照を取得する
    fn vals(&self) -> Ref<[Option<V>; 2 * D - 1]>;
    /// 値の配列への可変参照を取得する
    fn vals_mut(&mut self) -> RefMut<[Option<V>; 2 * D - 1]>;
    /// n番目の値への不変参照を取得する
    fn nth_val(&self, n: usize) -> Option<Ref<V>>;
    /// n番目の値への可変参照を取得する
    fn nth_val_mut(&mut self, n: usize) -> Option<RefMut<V>>;
    /// 親ノードへの不変参照を取得する
    fn parent(&self) -> Ref<Option<ParentPtr<D, K, V>>>;
    /// 親ノードへの可変参照を取得する
    fn parent_mut(&mut self) -> RefMut<Option<ParentPtr<D, K, V>>>;
    /// 子ノードの配列への不変参照を取得する
    fn children(&self) -> Ref<Option<[Option<NodePtr<D, K, V>>; 2 * D]>>;
    /// 子ノードの配列への可変参照を取得する
    fn children_mut(&mut self) -> RefMut<Option<[Option<NodePtr<D, K, V>>; 2 * D]>>;
    /// n番目の子ノードのポインタを取得する
    fn nth_child(&self, n: usize) -> Option<NodePtr<D, K, V>> {
        self.children()
            .as_ref()
            .and_then(|ch| ch.get(n).cloned())
            .flatten()
    }
    /// 葉ノードか判定する
    fn is_leaf(&self) -> bool;
    /// 空きが存在するか判定
    fn is_full(&self) -> bool {
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
    impl_get_ref!(parent, parent, Ref<Option<ParentPtr<D, K, V>>>);
    impl_get_ref!(parent_mut, parent, RefMut<Option<ParentPtr<D, K, V>>>, mut);
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

    fn nth_key(&self, n: usize) -> Option<Ref<K>> {
        Ref::filter_map(self.keys(), |arr| arr.get(n).map(|x| x.as_ref()).flatten()).ok()
    }
    fn nth_val(&self, n: usize) -> Option<Ref<V>> {
        Ref::filter_map(self.vals(), |arr| arr.get(n).map(|x| x.as_ref()).flatten()).ok()
    }
    fn nth_val_mut(&mut self, n: usize) -> Option<RefMut<V>> {
        RefMut::filter_map(self.vals_mut(), |arr| {
            arr.get_mut(n).map(|x| x.as_mut()).flatten()
        })
        .ok()
    }

    fn is_leaf(&self) -> bool {
        self.borrow().is_leaf()
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        btree,
        debug::print_as_tree,
        node::{BTreeNode, NodePtr},
    };

    use super::NodeUtil;

    #[test]
    fn test_nth_key() {
        let mut tree: Option<NodePtr<2, i32, &str>> = btree! {
            keys: [Some(0), Some(5), None],
            vals: [Some("hoge"), Some("fuga"), None],
            size: 2
        };

        print_as_tree(&tree);

        {
            let first_key = tree.as_ref().unwrap().nth_key(0);
            let second_key = tree.as_ref().unwrap().nth_key(1);
            let third_key = tree.as_ref().unwrap().nth_key(2);
            let fourth_key = tree.as_ref().unwrap().nth_key(3);

            assert_eq!(*first_key.unwrap(), 0);
            assert_eq!(*second_key.unwrap(), 5);
            assert!(third_key.is_none());
            assert!(fourth_key.is_none());
        }

        {
            let first_val = tree.as_ref().unwrap().nth_val(0);
            let second_val = tree.as_ref().unwrap().nth_val(1);
            let third_val = tree.as_ref().unwrap().nth_val(2);
            let fourth_val = tree.as_ref().unwrap().nth_val(3);

            assert_eq!(*first_val.unwrap(), "hoge");
            assert_eq!(*second_val.unwrap(), "fuga");
            assert!(third_val.is_none());
            assert!(fourth_val.is_none());
        }

        {
            let mut first_val_mut = tree.as_mut().unwrap().nth_val_mut(0);

            println!("before: {:?}", first_val_mut);

            **first_val_mut.as_mut().unwrap() = "piyo";

            println!("after: {:?}", first_val_mut);
        }

        print_as_tree(&tree);
    }
}
