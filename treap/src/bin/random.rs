use rand;
use treap::treap::*;
use rand::prelude::*;

fn main() {
    let mut tree = Treap::new();

    for _ in 0..20 {
        let x: i8 = random();
        println!("------------------");
        println!("Insert: {}", x);
        tree.insert(x);
        tree.pretty_print();

        let y: i8 = random();
        println!("------------------");
        let res = tree.search(&y);
        println!("Search: {}, found={}", y, res);
        tree.pretty_print();
    }
}
