//! ノードのユーティリティ

use crate::NodePtr;

pub trait NodeUtil<const D: usize, K, V>
where
    [(); 2 * D - 1]:,
{
    /// ノードの要素数を取得する
    fn size(&self) -> &usize;
    /// ノードの要素数への可変参照を取得する
    fn size_mut(&mut self) -> &mut usize;
    /// n番目のキーへの不変参照を取得する
    fn nth_key(&self, n: usize) -> Option<&K>;
    /// n番目の値への不変参照を取得する
    fn nth_val(&self, n: usize) -> Option<&V>;
    /// n番目の値への可変参照を取得する
    fn nth_val_mut(&mut self, n: usize) -> Option<&mut V>;
    /// n番目の子ノードへの不変参照を取得する
    fn nth_child(&self, n: usize) -> Option<&NodePtr<D, K, V>>;
    /// n番目の子ノードへの可変参照を取得する
    fn nth_child_mut(&mut self, n: usize) -> Option<&mut NodePtr<D, K, V>>;
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
            &self.$field
        }
    };
    ($name:ident, $field:ident, $return:ty, mut) => {
        fn $name(&mut self) -> $return {
            &mut self.$field
        }
    };
}

impl<const D: usize, K, V> NodeUtil<D, K, V> for NodePtr<D, K, V>
where
    [(); 2 * D - 1]:,
    [(); 2 * D]:,
{
    impl_get_ref!(size, size, &usize);
    impl_get_ref!(size_mut, size, &mut usize, mut);

    fn nth_key(&self, n: usize) -> Option<&K> {
        self.keys.get(n).map(|x| x.as_ref()).flatten()
    }
    fn nth_val(&self, n: usize) -> Option<&V> {
        self.vals.get(n).map(|x| x.as_ref()).flatten()
    }
    fn nth_val_mut(&mut self, n: usize) -> Option<&mut V> {
        self.vals.get_mut(n).map(|x| x.as_mut()).flatten()
    }
    fn nth_child(&self, n: usize) -> Option<&NodePtr<D, K, V>> {
        self.children.as_ref()?.get(n).map(|x| x.as_ref()).flatten()
    }
    fn nth_child_mut(&mut self, n: usize) -> Option<&mut NodePtr<D, K, V>> {
        self.children
            .as_mut()?
            .get_mut(n)
            .map(|x| x.as_mut())
            .flatten()
    }

    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
}

#[cfg(test)]
mod test {
    use crate::{btree, print_as_tree, BTreeNode, NodePtr};

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
