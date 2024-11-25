//! ノードのユーティリティ

use crate::node::{Internal, Leaf};

pub trait NodeUtil<const D: usize, K, V>
where
    [(); D + 1]:
{
    /// ノードの要素数を取得する
    fn size(&self) -> usize;
    /// ノードの要素数への可変参照を取得する
    fn size_mut(&mut self) -> &mut usize;
    /// キーの配列への参照を取得する
    fn keys(&self) -> &[Option<K>; D];
    /// キーの配列への可変参照を取得する
    fn keys_mut(&mut self) -> &mut [Option<K>; D];
    /// 値の配列への不変参照を取得する
    fn vals(&self) -> &[Option<V>; D];
    /// 値の配列への可変参照を取得する
    fn vals_mut(&mut self) -> &mut [Option<V>; D];
    /// キーと値両方への可変参照を取得する
    fn keys_and_vals_mut(&mut self) -> (&mut [Option<K>; D], &mut [Option<V>; D]);
}

impl<const D: usize, K, V> NodeUtil<D, K, V> for Leaf<D, K, V>
where 
    [(); D + 1]:
{
    fn size(&self) -> usize {
        self.size
    }
    fn size_mut(&mut self) -> &mut usize {
        &mut self.size
    }
    fn keys(&self) -> &[Option<K>; D] {
        &self.keys
    }
    fn keys_mut(&mut self) -> &mut [Option<K>; D] {
        &mut self.keys
    }
    fn vals(&self) -> &[Option<V>; D] {
        &self.vals
    }
    fn vals_mut(&mut self) -> &mut [Option<V>; D] {
        &mut self.vals
    }
    fn keys_and_vals_mut(&mut self) -> (&mut [Option<K>; D], &mut [Option<V>; D]) {
        (&mut self.keys, &mut self.vals)
    }
}

impl<const D: usize, K, V> NodeUtil<D, K, V> for Internal<D, K, V>
where
    [(); D + 1]:,
{
    fn size(&self) -> usize {
        self.size
    }
    fn size_mut(&mut self) -> &mut usize {
        &mut self.size
    }
    fn keys(&self) -> &[Option<K>; D] {
        &self.keys
    }
    fn keys_mut(&mut self) -> &mut [Option<K>; D] {
        &mut self.keys
    }
    fn vals(&self) -> &[Option<V>; D] {
        &self.vals
    }
    fn vals_mut(&mut self) -> &mut [Option<V>; D] {
        &mut self.vals
    }
    fn keys_and_vals_mut(&mut self) -> (&mut [Option<K>; D], &mut [Option<V>; D]) {
        (&mut self.keys, &mut self.vals)
    }
}
