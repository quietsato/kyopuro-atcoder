#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

fn main() {
    let (h, w, k): (usize, usize, usize) = parse_line().unwrap();
    let c: Vec<Vec<_>> = {
        let mut c = vec![];
        for _ in 0..h {
            c.push(parse_line::<String>().unwrap().as_bytes().to_owned());
        }
        c
    };

    let ans = {
        let mut ans = 0;

        for row in 0..2usize.pow(h as u32) {
            for col in 0..2usize.pow(w as u32) {
                let mut black = 0;
                
                for y in 0..h {
                    for x in 0..w {
                        if row & (1 << y) == 0 && col & (1 << x) == 0 && c[y][x] == b'#' {
                            black += 1;
                        }
                    }
                }

                if black == k {
                    ans += 1;
                }
            }
        }

        ans
    };

    println!("{}", ans);
}
