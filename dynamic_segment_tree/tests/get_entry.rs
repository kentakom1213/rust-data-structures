use dynamic_segment_tree::{alg::monoids::Add, dynamic_segment_tree::DynamicSegmentTree};

#[test]
fn test_get() {
    let mut seg = DynamicSegmentTree::<isize, Add>::new();

    // {0: -700}
    seg.insert(0, -700);
    seg.print_as_binary_tree();

    assert_eq!(seg.get(&-200_000), &0);
    assert_eq!(seg.get(&0), &-700);
    assert_eq!(seg.get(&1_000_000), &0);
    assert_eq!(seg.get(&998244353), &0);
    assert_eq!(seg.get(&1_000_000_007), &0);

    // {0: -700, 1000000: 15}
    seg.insert(1_000_000, 15);
    seg.print_as_binary_tree();

    assert_eq!(seg.get(&-200_000), &0);
    assert_eq!(seg.get(&0), &-700);
    assert_eq!(seg.get(&1_000_000), &15);
    assert_eq!(seg.get(&998244353), &0);
    assert_eq!(seg.get(&1_000_000_007), &0);

    // {-200000: 77, 0: -700, 1000000: 15}
    seg.insert(-200_000, 77);
    seg.print_as_binary_tree();

    assert_eq!(seg.get(&-200_000), &77);
    assert_eq!(seg.get(&0), &-700);
    assert_eq!(seg.get(&1_000_000), &15);
    assert_eq!(seg.get(&998244353), &0);
    assert_eq!(seg.get(&1_000_000_007), &0);

    // {-200000: 77, 0: -700, 1000000: 15, 998244353: 400}
    seg.insert(998244353, 400);
    seg.print_as_binary_tree();

    assert_eq!(seg.get(&-200_000), &77);
    assert_eq!(seg.get(&0), &-700);
    assert_eq!(seg.get(&1_000_000), &15);
    assert_eq!(seg.get(&998244353), &400);
    assert_eq!(seg.get(&1_000_000_007), &0);
}

#[test]
fn test_entry() {
    let mut seg = DynamicSegmentTree::<isize, Add>::new();
    seg.print_as_binary_tree();

    // {200: 200}
    {
        let mut ent = seg.get_mut(200);
        println!("{ent:?}");
        assert_eq!(*ent, 0);
        *ent = 200;
        assert_eq!(*ent, 200);
    }
    seg.print_as_binary_tree();
    assert_eq!(seg.get_range(..), 200);

    // {-1: 3, 200: 200}
    {
        let mut ent = seg.get_mut(-1);
        println!("{ent:?}");
        assert_eq!(*ent, 0);
        *ent = 3;
        assert_eq!(*ent, 3);
    }
    seg.print_as_binary_tree();
    assert_eq!(seg.get_range(..), 203);

    // {-1: 3, 55: 700, 200: 200}
    {
        let mut ent = seg.get_mut(55);
        println!("{ent:?}");
        assert_eq!(*ent, 0);
        *ent = 700;
        assert_eq!(*ent, 700);
    }
    seg.print_as_binary_tree();
    assert_eq!(seg.get_range(..), 903);

    // {-1: 3, 55: 700, 200: 20}
    {
        let mut ent = seg.get_mut(200);
        println!("{ent:?}");
        assert_eq!(*ent, 200);
        *ent /= 10;
        assert_eq!(*ent, 20);
    }
    seg.print_as_binary_tree();
    assert_eq!(seg.get_range(..), 723);

    // {-1: 303, 55: 700, 200: 20}
    {
        let mut ent = seg.get_mut(-1);
        println!("{ent:?}");
        assert_eq!(*ent, 3);
        *ent += 300;
        assert_eq!(*ent, 303);
    }
    seg.print_as_binary_tree();
    assert_eq!(seg.get_range(..), 1023);
}
