use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: u64,
        ca: [(u64, [u64; m]); n]
    }

    let ans = {
        let mut ans: Option<u64> = None;
        for b in 0..2u32.pow(12) {
            let mut understanding = vec![0; m];
            let mut cost = 0;
            for (i, (c, a)) in ca.iter().enumerate() {
                if (b >> i) & 1 == 1 {
                    cost += c;
                    understanding.iter_mut().zip(a.iter()).for_each(|(u, a)| {
                        *u += *a;
                    })
                }
            }
            if understanding.iter().all(|u| u >= &x) {
                if let Some(a) = ans {
                    ans = Some(a.min(cost));
                } else {
                    ans = Some(cost);
                }
            }
        }
        ans
    };

    println!("{}", ans.map_or_else(|| (-1).to_string(), |a| { a.to_string() }));
}
