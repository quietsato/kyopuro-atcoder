use std::slice::RSplitMut;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n]
    }

    let ts = ab.iter().map(|(a, b)| a / b).collect_vec();
    let mut t = ts.iter().sum::<f64>() / 2.0;

    let mut ans = 0.0;
    for i in 0.. {
        if t < ts[i] {
            ans += ab[i].1 * t;
            break;
        }
        ans += ab[i].0 as f64;
        t -= ts[i];
    }
    
    println!("{}", ans);
}
