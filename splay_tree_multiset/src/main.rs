use splay_tree_multiset::multiset::SplayTreeMultiSet;

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
    let mut multiset = SplayTreeMultiSet::<i8>::new();

    loop {
        let (t, x) = get!(i8, i8);

        if t == 0 {
            multiset.insert(x);
            println!("--- insert {} ---", x);
        } else {
            multiset.delete(&x);
            println!("--- delete {} ---", x);
        }

        println!("{:#?}", multiset);
    }
}
