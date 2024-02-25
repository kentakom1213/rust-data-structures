use dynamic_segment_tree::{alg::monoids::Add, dynamic_segment_tree::DynamicSegmentTree};
use rand::{
    distributions::{Alphanumeric, DistString},
    random, thread_rng,
};

const ITER: usize = 20;
const SIZE: usize = 5;

fn main() {
    let mut rng = thread_rng();

    let mut seg = DynamicSegmentTree::<String, Add>::new();

    for _ in 0..ITER {
        let val = random::<i32>() as isize;

        // 挿入
        let key = Alphanumeric.sample_string(&mut rng, SIZE);
        seg.insert(key, val);
    }

    // 表示
    seg.print_as_binary_tree();
    seg.print_as_btree();
}
