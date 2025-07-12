use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

#[derive(Debug, Clone)]
enum Op {
    Replace { p: usize, offset: usize },
    PushStr { s: String },
}

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut ops = vec![vec![Op::PushStr { s: String::new() }]; n + 1];

    for _ in 0..q {
        input! { t: u8 }
        match t {
            1 => {
                input! { p: usize }
                let l = ops[0].len();
                ops[p].push(Op::Replace {
                    p: 0,
                    offset: l - 1,
                });
            }
            2 => {
                input! { p: usize, s: String }
                ops[p].push(Op::PushStr { s });
            }
            3 => {
                input! { p: usize }
                let l = ops[p].len();
                ops[0].push(Op::Replace {
                    p: p,
                    offset: l - 1,
                })
            }
            _ => unreachable!(),
        }
    }

    let mut p = 0;
    let mut offset = ops[0].len() - 1;
    let mut ans = vec![];

    while offset > 0 {
        match &ops[p][offset] {
            Op::Replace { p: pi, offset: oi } => {
                p = *pi;
                offset = *oi;
            }
            Op::PushStr { s } => {
                ans.push(s);
                offset -= 1;
            }
        }
    }

    println!("{}", ans.iter().rev().join(""));
}
