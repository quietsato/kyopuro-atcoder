#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

fn main() {
    let n: usize = parse_line().unwrap();
    let s: String = parse_line().unwrap();
    let s = s.as_bytes();

    let ans = {
        let (r, g, b) = {
            let (mut r, mut g, mut b) = (vec![], vec![], vec![]);
            for i in 0..n {
                match s[i as usize] {
                    b'R' => r.push(i),
                    b'G' => g.push(i),
                    b'B' => b.push(i),
                    _ => unreachable!(),
                }
            }
            (r, g, b)
        };

        let mut ans = r.len() * g.len() * b.len();

        // 条件1を満たしつつも条件2を満たさない場合をansから除く
        for i in 0..n {
            for j in i + 2..n {
                if (i + j) & 1 == 1 {
                    continue;
                }
                let k = (i + j) / 2;

                if s[i] != s[k] && s[k] != s[j] && s[i] != s[j] {
                    ans -= 1;
                }
            }
        }

        ans
    };

    println!("{}", ans);
}
