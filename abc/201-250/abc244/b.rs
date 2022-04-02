use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        t: Chars
    }
    let diff = &[(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut ans = (0i64, 0i64);
    let mut i = 0usize;
    for t in t {
        match t {
            'S' => {
                ans.0 += diff[i].0;
                ans.1 += diff[i].1;
            }
            'R' => {
                i += 1;
                i %= 4;
            },
            _ => unreachable!()
        }
    }
    println!("{} {}", ans.0, ans.1);
}

