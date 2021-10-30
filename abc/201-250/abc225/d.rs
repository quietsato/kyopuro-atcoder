use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize
    }

    let mut root = (0..n).collect_vec();
    let mut next = (0..n).collect_vec();

    for _ in 0..q {
        input! {
            q: usize
        }

        if q == 1 {
            input! {
                x: Usize1,
                y: Usize1
            }
            next[x] = y;
            root[y] = x;
        } else if q == 2 {
            input! {
                x: Usize1,
                y: Usize1
            }
            next[x] = x;
            root[y] = y;
        } else if q == 3 {
            input! {
                x: Usize1
            }
            let mut ans = vec![];
            let mut r = root[x];
            while root[r] != r {
                r = root[r];
            }
            ans.push(r + 1);
            let mut o = r;
            while next[o] != o {
                o = next[o];
                ans.push(o + 1);
            }
            println!("{} {}", ans.len(), ans.iter().join(" "));
        }
    }
}
