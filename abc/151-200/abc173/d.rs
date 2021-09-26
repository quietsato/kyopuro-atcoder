use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }

    a.sort();

    let ans = (1..n).map(|i| a[(n - 1) - (i / 2)]).sum::<u64>();

    println!("{}", ans);
}
