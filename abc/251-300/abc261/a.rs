use proconio::input;

fn main() {
    input! {
        l1: usize,
        r1: usize,
        l2: usize,
        r2: usize,
    }

    let ans = (0..=r1.max(r2))
        .filter(|i| (l1..r1).contains(i) && (l2..r2).contains(i))
        .count();

    println!("{}", ans);
}
