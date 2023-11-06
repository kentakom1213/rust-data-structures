use splay_tree_multiset::multiset::MultiSet;

macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
}

fn main() {
    let mut multiset = MultiSet::<i8>::new();

    loop {
        let (t, x) = get!(i8, i8);

        match (t, x) {
            (0, x) => {
                multiset.insert(x);
                println!("--- insert {} ---", x);
            }
            (1, x) => {
                multiset.delete(&x);
                println!("--- delete {} ---", x);
            }
            (2, x) => {
                multiset.lower_bound(&x);
                println!("--- lower bound {} ---", x);
            }
            (3, x) => {
                multiset.upper_bound(&x);
                println!("--- upper bound {} ---", x);
            }
            _ => (),
        }

        println!("{:#?}", multiset);
    }
}
