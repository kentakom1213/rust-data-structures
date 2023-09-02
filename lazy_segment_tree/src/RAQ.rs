#![allow(dead_code)]
#![allow(unused_variables)]

// データ型
type T = isize;

// 単位元
const I: T = 0;

/// ## LazySegmentTree
/// - 遅延セグメント木
/// - 区間加算、区間取得をサポートする
/// - 対応している型はisizeのみ（ジェネリクスなし）
#[derive(Debug)]
pub struct LazySegmentTree {
    size: usize,
    offset: usize,
    data: Vec<T>,
    lazy: Vec<T>,
}

impl LazySegmentTree {
    /// 新規作成
    pub fn new(n: usize) -> Self {
        let offset = n.next_power_of_two();
        Self {
            size: n,
            offset,
            data: vec![I; offset << 1],
            lazy: vec![I; offset << 1],
        }
    }

    /// 一点加算
    /// - 0-indexed
    pub fn set_point(&mut self, mut idx: usize, val: T) {
        idx += self.offset;
        // 根に到達するまで更新
        while idx > 0 {
            // 加算
            self.data[idx] += val;
            idx >>= 1; // 2で割る
        }
    }

    /// 遅延値を評価
    fn eval(&mut self, idx: usize) {
        // 葉でなければ子に伝搬
        if idx < self.offset - 1 {
            self.lazy[idx * 2 + 1] += self.lazy[idx];
            self.lazy[idx * 2 + 2] += self.lazy[idx];
        }
        // 自身を更新
        self.data[idx] += self.lazy[idx];
        self.lazy[idx] = I;
    }

    /// 区間加算
    /// - [left, right)
    pub fn set_range(&mut self, left: usize, right: usize, val: T) {

    }

    /// 区間取得
    /// - 再帰実装
    /// - [left, right)
    pub fn get_range_rec(&mut self, left: usize, right: usize) -> T {
        self.get_range_sub(left, right, 0, self.offset, 0)
    }

    fn get_range_sub(&mut self, left: usize, right: usize, begin: usize, end: usize, idx: usize) -> T {
        // 区間を含まない
        if end <= left || right <= begin {
            I
        }
        // 区間を包含する
        else if left <= begin && end <= right {
            self.data[idx]
        }
        // 区間が重なる
        else {
            let mid = (begin + end) / 2;
            let l_val = self.get_range_sub(left, right, begin, mid, idx * 2 + 1);
            let r_val = self.get_range_sub(left, right, mid, end, idx * 2 + 2);
            l_val + r_val
        }
    }

    /// 区間取得
    /// - 非再帰実装
    /// - [left, right)
    pub fn get_range_non_rec(&self, mut left: usize, mut right: usize) -> T {
        left += self.offset;
        right += self.offset;
        let mut res = T::default(); // 解を保存

        while left < right {
            if left & 1 == 1 {
                res += self.data[left];
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                res += self.data[right];
            }
            left >>= 1;
            right >>= 1;
        }
        res
    }
}

#[cfg(test)]
mod test_lazy_segtree {
    use super::*;

    #[test]
    fn test_get_range_non_rec() {
        let mut lazy_segtree = LazySegmentTree::new(4);
        println!("{:?}", &lazy_segtree.data[4..8]);

        // 一点加算
        lazy_segtree.set_point(0, 5);
        println!("{:?}", &lazy_segtree.data[4..8]);

        // 区間取得
        println!("[0, 1) -> {:?}", lazy_segtree.get_range_non_rec(0, 1));
        println!("[1, 3) -> {:?}", lazy_segtree.get_range_non_rec(1, 3));
        println!("[1, 4) -> {:?}", lazy_segtree.get_range_non_rec(1, 4));
        println!("[0, 4) -> {:?}", lazy_segtree.get_range_non_rec(0, 4));

        // 一点加算
        lazy_segtree.set_point(3, -2);
        println!("{:?}", &lazy_segtree.data[4..8]);

        // 区間取得
        println!("[0, 1) -> {:?}", lazy_segtree.get_range_non_rec(0, 1));
        println!("[1, 3) -> {:?}", lazy_segtree.get_range_non_rec(1, 3));
        println!("[1, 4) -> {:?}", lazy_segtree.get_range_non_rec(1, 4));
        println!("[0, 4) -> {:?}", lazy_segtree.get_range_non_rec(0, 4));
    }

    #[test]
    fn test_get_range_rec() {
        let mut lazy_segtree = LazySegmentTree::new(4);
        println!("{:?}", &lazy_segtree.data[4..8]);

        // 一点加算
        lazy_segtree.set_point(0, 5);
        println!("{:?}", &lazy_segtree.data[4..8]);

        // 区間取得
        println!("[0, 1) -> {:?}", lazy_segtree.get_range_rec(0, 1));
        println!("[1, 3) -> {:?}", lazy_segtree.get_range_rec(1, 3));
        println!("[1, 4) -> {:?}", lazy_segtree.get_range_rec(1, 4));
        println!("[0, 4) -> {:?}", lazy_segtree.get_range_rec(0, 4));

        // 一点加算
        lazy_segtree.set_point(3, -2);
        println!("{:?}", &lazy_segtree.data[4..8]);

        // 区間取得
        println!("[0, 1) -> {:?}", lazy_segtree.get_range_rec(0, 1));
        println!("[1, 3) -> {:?}", lazy_segtree.get_range_rec(1, 3));
        println!("[1, 4) -> {:?}", lazy_segtree.get_range_rec(1, 4));
        println!("[0, 4) -> {:?}", lazy_segtree.get_range_rec(0, 4));
    }
}
