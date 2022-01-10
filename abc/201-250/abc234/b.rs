use itertools::Itertools;
use num_traits::Float;
use proconio::input;

macro_rules! chmax {
    ($target:expr, $($v: expr),+) => {{
        $target = $target$(.max($v))*;
    }};
}

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    }

    let mut ans = 0.0;
    for ps in xy.iter().combinations(2) {
        let a = ps[0];
        let b = ps[1];
        chmax!(ans, ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt());
    }

    println!("{}", ans);
}
