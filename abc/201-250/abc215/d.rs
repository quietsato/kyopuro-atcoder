use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n]
    }

    let mut ok = vec![true; m];
    let mut did = vec![false; 100_000];

    for a in a {
        let p = prime_factorization(a);
        for p in p {
            let p = p as usize;
            if !did[p - 1] {
                for i in (p - 1..).step_by(p).take_while(|&i| i < m) {
                    ok[i] = false;
                }
                did[p - 1] = true;
            }
        }
    }

    let ans = (1..)
        .zip(&ok)
        .filter_map(|(i, ok)| if *ok { Some(i) } else { None })
        .collect_vec();
    println!("{}", ans.len());
    for ans in &ans {
        println!("{}", ans);
    }
}

fn prime_factorization(x: u64) -> Vec<u64> {
    if x < 2 {
        vec![]
    } else {
        let (mut x, mut res) = (x, vec![]);
        for i in 2..(x as f64).sqrt().ceil() as u64 {
            while x % i == 0 {
                x /= i;
                res.push(i);
            }
        }
        if x != 1 {
            res.push(x);
        }
        res
    }
}
