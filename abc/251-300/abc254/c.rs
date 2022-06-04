use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n]
    }

    if k == 1 {
        println!("Yes");
        return;
    }

    let sorted = a.iter().sorted().collect_vec();

    let mut m = vec![];
    for i in 0..k {
        m.push(a.iter().skip(i).step_by(k).sorted().collect_vec());
    }
    for i in 0..n {
        if sorted[i] != m[i % k][i / k] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
