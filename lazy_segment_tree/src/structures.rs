use crate::alg::Monoid;

/// ## RAQ
/// - 区間加算
/// - 区間和
#[derive(Debug)]
pub struct RAQ;

impl Monoid for RAQ {
    type X = isize;
    type M = isize;
    const IX: Self::X = 0;
    const IM: Self::M = 0;
    fn fx(x: &Self::X, y: &Self::X) -> Self::X {
        x + y
    }
    fn fa(x: &Self::X, y: &Self::M) -> Self::X {
        x + y
    }
    fn fm(x: &Self::M, y: &Self::M) -> Self::M {
        x + y
    }
    fn fp(x: &Self::M, p: usize) -> Self::M {
        x * p as isize
    }
}

/// ## RMQandRUQ
/// - 区間更新
/// - 区間最小値
#[derive(Debug)]
pub struct RMQandRUQ;

impl Monoid for RMQandRUQ {
    type X = isize;
    type M = isize;
    const IM: Self::M = (1 << 31) - 1;
    const IX: Self::X = (1 << 31) - 1;
    fn fx(x: &Self::X, y: &Self::X) -> Self::X {
        *x.min(y)
    }
    fn fa(_x: &Self::X, y: &Self::M) -> Self::X {
        *y
    }
    fn fm(_x: &Self::M, y: &Self::M) -> Self::M {
        *y
    }
    fn fp(x: &Self::M, _p: usize) -> Self::M {
        *x
    }
}


#[cfg(test)]
mod test_abstract_lazy_segment_tree {
    use super::*;
    use crate::lazy_segment_tree::LazySegmentTree;

    /// 区間加算・区間和のテスト
    #[test]
    fn test_raq() {
        // 遅延セグメント木
        let mut segtree = LazySegmentTree::<RAQ>::new(4);
        // -> [0, 0, 0, 0]

        segtree.set_range(0, 2, 4);
        // -> [4, 4, 0, 0]

        println!("{:?}", &segtree);

        assert_eq!(segtree.get_range(0, 2), 8);
        assert_eq!(segtree.get_range(0, 4), 8);
        assert_eq!(segtree.get_range(1, 3), 4);
        assert_eq!(segtree.get_range(2, 4), 0);

        segtree.set_range(1, 3, 1);
        // -> [4, 5, 1, 0]

        println!("{:?}", &segtree);

        assert_eq!(segtree.get_range(0, 2), 9);
        assert_eq!(segtree.get_range(0, 4), 10);
        assert_eq!(segtree.get_range(1, 3), 6);
        assert_eq!(segtree.get_range(2, 4), 1);

        segtree.set_range(1, 4, -3);
        // -> [4, 2, -2, -3]

        println!("{:?}", &segtree);

        assert_eq!(segtree.get_range(0, 2), 6);
        assert_eq!(segtree.get_range(0, 4), 1);
        assert_eq!(segtree.get_range(1, 3), 0);
        assert_eq!(segtree.get_range(2, 4), -5);
    }

    /// 区間更新・区間最小値のテスト
    #[test]
    fn test_rmq_and_ruq() {
        const INF: isize = (1 << 31) - 1;

        // 遅延セグメント木
        let mut segtree = LazySegmentTree::<RMQandRUQ>::new(4);
        // -> [INF, INF, INF, INF]

        segtree.set_range(0, 2, 4);
        // -> [4, 4, INF, INF]

        println!("{:?}", &segtree);

        // assert_eq!(segtree.get_range(0, 2), 4);
        segtree.get_range(0, 2);
        println!("{:?}", &segtree);
        assert_eq!(segtree.get_range(0, 4), 4);
        assert_eq!(segtree.get_range(1, 3), 4);
        assert_eq!(segtree.get_range(2, 4), INF);

        segtree.set_range(1, 3, 1);
        // -> [4, 1, 1, INF]

        println!("{:?}", &segtree);

        assert_eq!(segtree.get_range(0, 2), 1);
        assert_eq!(segtree.get_range(0, 4), 1);
        assert_eq!(segtree.get_range(1, 3), 1);
        assert_eq!(segtree.get_range(2, 4), 1);

        segtree.set_range(1, 4, 5);
        // -> [4, 5, 5, 5]

        println!("{:?}", &segtree);

        assert_eq!(segtree.get_range(0, 2), 4);
        assert_eq!(segtree.get_range(0, 4), 4);
        assert_eq!(segtree.get_range(1, 3), 5);
        assert_eq!(segtree.get_range(2, 4), 5);
    }
}
