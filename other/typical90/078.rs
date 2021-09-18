use proconio::{input, marker::Usize1};

// 078 Easy Graph Problem (★2)
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m]
    }

    let mut adj_count = vec![0u32; n];

    for &(a, b) in &ab {
        let (a, _b) = (a.max(b), a.min(b));
        adj_count[a] += 1;
    }

    let ans = adj_count.iter().filter(|&&c| c == 1).count();
    println!("{}", ans);
}
