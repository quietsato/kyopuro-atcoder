#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

fn main() {
    let s: Vec<char> = parse_line::<String>().unwrap().chars().collect();

    let x: String = {
        let chunks = s.split_at(1);

        vec![chunks.1, chunks.0].concat().iter().collect()
    };

    println!("{}", x);
}
