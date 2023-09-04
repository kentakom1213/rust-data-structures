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

    /// 遅延値を評価
    fn eval(&mut self, idx: usize, len: usize) {
        // 葉でなければ子に伝搬
        if idx < self.offset {
            self.lazy[idx * 2] += self.lazy[idx];
            self.lazy[idx * 2 + 1] += self.lazy[idx];
        }
        // 自身を更新
        self.data[idx] += self.lazy[idx] * len as isize;
        self.lazy[idx] = I;
    }

    /// 区間加算
    /// - [left, right)
    pub fn set_range(&mut self, left: usize, right: usize, val: T) {
        self.set_range_sub(left, right, val, 0, self.offset, 1);
    }

    fn set_range_sub(&mut self, left: usize, right: usize, val: T, begin: usize, end: usize, idx: usize) {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を内包するとき
        if left <= begin && end <= right {
            self.lazy[idx] += val;
            self.eval(idx, end - begin);
        }
        // 区間が重なるとき
        else if left < end && begin < right {
            let mid = (begin + end) / 2;
            // 左の子を更新
            self.set_range_sub(left, right, val, begin, mid, idx * 2);
            // 右の子を更新
            self.set_range_sub(left, right, val, mid, end, idx * 2 + 1);
            // 値を更新
            self.data[idx] = self.data[idx * 2] + self.data[idx * 2 + 1];
        }
    }

    /// 区間取得
    /// - 再帰実装
    /// - [left, right)
    pub fn get_range(&mut self, left: usize, right: usize) -> T {
        self.get_range_sub(left, right, 0, self.offset, 1)
    }

    fn get_range_sub(&mut self, left: usize, right: usize, begin: usize, end: usize, idx: usize) -> T {
        // 遅延値を評価
        self.eval(idx, end - begin);
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
            let l_val = self.get_range_sub(left, right, begin, mid, idx * 2);
            let r_val = self.get_range_sub(left, right, mid, end, idx * 2 + 1);
            l_val + r_val
        }
    }
}

#[cfg(test)]
mod test_lazy_segtree {
    use super::*;

    #[test]
    fn test_get_range() {
        let mut lazy_segtree = LazySegmentTree::new(4);
        // -> [0, 0, 0, 0]
        println!("{:?}", &lazy_segtree);

        // 区間加算
        lazy_segtree.set_range(0, 3, 5);
        // -> [5, 5, 5, 0]
        println!("{:?}", &lazy_segtree);

        // 区間取得
        assert_eq!(lazy_segtree.get_range(0, 1), 5);
        assert_eq!(lazy_segtree.get_range(1, 3), 10);
        assert_eq!(lazy_segtree.get_range(1, 4), 10);
        assert_eq!(lazy_segtree.get_range(0, 4), 15);

        // 区間加算
        lazy_segtree.set_range(2, 4, -2);
        // -> [5, 5, 3, -2]
        println!("{:?}", &lazy_segtree);

        // 区間取得
        assert_eq!(lazy_segtree.get_range(0, 1), 5);
        assert_eq!(lazy_segtree.get_range(1, 3), 8);
        assert_eq!(lazy_segtree.get_range(1, 4), 6);
        assert_eq!(lazy_segtree.get_range(0, 4), 11);
    }

    #[test]
    fn test_size_1() {
        let mut lazy_segtree = LazySegmentTree::new(1);
        // -> [0]
        println!("{:?}", &lazy_segtree);

        assert_eq!(lazy_segtree.get_range(0, 1), 0);

        lazy_segtree.set_range(0, 1, 1);
        // -> [1]
        println!("{:?}", &lazy_segtree);

        assert_eq!(lazy_segtree.get_range(0, 1), 1);
    }
}
