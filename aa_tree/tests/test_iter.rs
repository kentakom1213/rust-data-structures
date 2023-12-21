use aa_tree::map::AATreeMap;

#[test]
fn test_insert() {
    let mut map = AATreeMap::new();

    let alphabet = [
        "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota", "kappa",
        "lambda", "mu", "nu", "xi", "omicron", "pi", "rho", "sigma", "tau", "upsilon", "phi",
        "chi", "psi", "omega",
    ];

    for (i, &a) in alphabet.iter().rev().enumerate() {
        map.insert(a, i);
    }

    map.pretty_print();

    let res: Vec<(&str, usize)> = map.iter().map(|(&k, &v)| (k, v)).collect();

    assert_eq!(
        res,
        vec![
            ("alpha", 23),
            ("beta", 22),
            ("chi", 2),
            ("delta", 20),
            ("epsilon", 19),
            ("eta", 17),
            ("gamma", 21),
            ("iota", 15),
            ("kappa", 14),
            ("lambda", 13),
            ("mu", 12),
            ("nu", 11),
            ("omega", 0),
            ("omicron", 9),
            ("phi", 3),
            ("pi", 8),
            ("psi", 1),
            ("rho", 7),
            ("sigma", 6),
            ("tau", 5),
            ("theta", 16),
            ("upsilon", 4),
            ("xi", 10),
            ("zeta", 18)
        ]
    );
}
