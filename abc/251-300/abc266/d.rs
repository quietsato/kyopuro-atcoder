use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

macro_rules! chmax {
    ($target:expr, $($v: expr),+) => {{
        $target = $target$(.max($v))*;
    }};
}

fn main() {
    input! {
        n: usize,
        mut txa: [(u64, usize, u64); n],
    };
    let t_max = txa.last().unwrap().0;
    let txa = {
        let mut map = BTreeMap::new();
        for (t, x, a) in txa {
            map.insert(t, (x, a));
        }
        map
    };
    let mut dp = vec![vec![0; 5]; t_max as usize + 1];
    for t in 1..=t_max as usize {
        for p in 0..=4 {
            if p > t {
                continue;
            }
            chmax!(
                dp[t][p],
                if p > 0 { dp[t - 1][p - 1] } else { 0 },
                if t > p { dp[t - 1][p] } else { 0 },
                if p < 4 && t > p + 1 {
                    dp[t - 1][p + 1]
                } else {
                    0
                }
            );
            if let Some(&(x, a)) = txa.get(&(t as u64)) {
                if x == p {
                    dp[t][p] += a;
                }
            }
        }
    }
    let ans = dp.last().unwrap().iter().max().unwrap();
    println!("{}", ans);
}
