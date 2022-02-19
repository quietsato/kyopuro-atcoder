use std::collections::BTreeSet;

use proconio::input;
use std::iter::FromIterator;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64
    }

    let mut primes = vec![];
    for i in 2..=200 {
        if primes.iter().all(|p| i % p != 0) {
            primes.push(i);
        }
    }

    let primes = BTreeSet::from_iter(primes.into_iter());

    let mut must_be_prime = true;
    for i in a..=b {
        if primes.range(i + c..=i + d).next().is_none() {
            must_be_prime = false;
        }
    }

    if must_be_prime {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}
