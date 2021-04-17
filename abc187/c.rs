use std::collections::BTreeSet;

#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

fn main() {
    let n: usize = parse_line().unwrap();
    let s: Vec<String> = {
        (0..n)
            .into_iter()
            .map(|_i| parse_line::<String>().unwrap())
            .collect()
    };

    if let Some(ans) = {
        let mut h = BTreeSet::new();
        s.iter()
            .filter_map(|s| match s.chars().nth(0).unwrap() {
                '!' if h.contains(&s[1..].to_string()) => Some(s[1..].to_string()),
                _ if h.contains(&format!("!{}", s)) => Some(s.clone()),
                _ => {
                    h.insert(s);
                    None
                }
            })
            .next()
    } {
        println!("{}", ans);
    } else {
        println!("satisfiable");
    }
}
