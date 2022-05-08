use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut x: [Usize1; q]
    }

    let mut arr = (0..n).collect_vec();
    let mut idx = (0..n).collect_vec();

    for x in x {
        let i1 = idx[x];
        let i2 = if i1 == n-1 {i1 - 1} else {i1 + 1 };
        let a1 = arr[i1];
        let a2 = arr[i2];
        arr[i1] = a2;
        arr[i2] = a1;
        idx[a2] = i1;
        idx[a1] = i2;
    }

    println!("{}", arr.iter().map(|n| (n + 1).to_string()).join(" "));
}
