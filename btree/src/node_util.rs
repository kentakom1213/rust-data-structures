//! ノードのユーティリティ

use crate::node::{Internal, Leaf};

pub trait NodeUtil<const D: usize, K, V>
where
    [(); 2 * D - 1]:,
{
    /// ノードの要素数を取得する
    fn size(&self) -> usize;
    /// ノードの要素数への可変参照を取得する
    fn size_mut(&mut self) -> &mut usize;
    /// 空きが存在するか判定
    fn is_filled(&self) -> bool {
        self.size() == 2 * D - 1
    }
    /// キーの配列への参照を取得する
    fn keys(&self) -> &[Option<K>; 2 * D - 1];
    /// キーの配列への可変参照を取得する
    fn keys_mut(&mut self) -> &mut [Option<K>; 2 * D - 1];
    /// 値の配列への不変参照を取得する
    fn vals(&self) -> &[Option<V>; 2 * D - 1];
    /// 値の配列への可変参照を取得する
    fn vals_mut(&mut self) -> &mut [Option<V>; 2 * D - 1];
    /// キーと値両方への可変参照を取得する
    fn keys_and_vals_mut(&mut self) -> (&mut [Option<K>; 2 * D - 1], &mut [Option<V>; 2 * D - 1]);
}

impl<const D: usize, K, V> NodeUtil<D, K, V> for Leaf<D, K, V>
where
    [(); 2 * D - 1]:,
{
    fn size(&self) -> usize {
        self.size
    }
    fn size_mut(&mut self) -> &mut usize {
        &mut self.size
    }
    fn keys(&self) -> &[Option<K>; 2 * D - 1] {
        &self.keys
    }
    fn keys_mut(&mut self) -> &mut [Option<K>; 2 * D - 1] {
        &mut self.keys
    }
    fn vals(&self) -> &[Option<V>; 2 * D - 1] {
        &self.vals
    }
    fn vals_mut(&mut self) -> &mut [Option<V>; 2 * D - 1] {
        &mut self.vals
    }
    fn keys_and_vals_mut(&mut self) -> (&mut [Option<K>; 2 * D - 1], &mut [Option<V>; 2 * D - 1]) {
        (&mut self.keys, &mut self.vals)
    }
}

impl<const D: usize, K, V> NodeUtil<D, K, V> for Internal<D, K, V>
where
    [(); 2 * D - 1]:,
{
    fn size(&self) -> usize {
        self.size
    }
    fn size_mut(&mut self) -> &mut usize {
        &mut self.size
    }
    fn keys(&self) -> &[Option<K>; 2 * D - 1] {
        &self.keys
    }
    fn keys_mut(&mut self) -> &mut [Option<K>; 2 * D - 1] {
        &mut self.keys
    }
    fn vals(&self) -> &[Option<V>; 2 * D - 1] {
        &self.vals
    }
    fn vals_mut(&mut self) -> &mut [Option<V>; 2 * D - 1] {
        &mut self.vals
    }
    fn keys_and_vals_mut(&mut self) -> (&mut [Option<K>; 2 * D - 1], &mut [Option<V>; 2 * D - 1]) {
        (&mut self.keys, &mut self.vals)
    }
}
