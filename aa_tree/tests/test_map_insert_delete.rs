use aa_tree::map::AATreeMap;

#[test]
fn test_insert() {
    let mut map = AATreeMap::new();

    let alphabet = [
        "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota", "kappa",
        "lambda", "mu", "nu", "xi", "omicron", "pi", "rho", "sigma", "tau", "upsilon", "phi",
        "chi", "psi", "omega",
    ];

    for (i, &a) in alphabet.iter().enumerate() {
        map.insert(a, i);
    }

    map.pretty_print();
}
