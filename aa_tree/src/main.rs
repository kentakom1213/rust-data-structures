use aa_tree::map::AATreeMap;
use std::io;

fn get_command() -> Option<(char, Option<isize>)> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok()?;

    let mut iter = line.trim().split_whitespace();
    let key = iter.next()?.parse().ok()?;
    let val = iter.next().and_then(|v| v.parse().ok());
    Some((key, val))
}

fn main() {
    let mut tree: AATreeMap<isize, Option<u8>> = AATreeMap::new();

    loop {
        tree.pretty_print();

        match get_command() {
            Some(('i', Some(v))) => {
                tree.insert(v, None);
            }
            Some(('d', Some(v))) => {
                tree.remove(&v);
            }
            Some(('c', _)) => {
                eprintln!("clear!");
                tree.clear();
            }
            _ => (),
        }
    }
}
