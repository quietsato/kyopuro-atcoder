use proconio::input;

fn main() {
    input! {
        mut n: u64
    }
    let mut ans = -1;
    while n > 0 {
        ans += 1;
        n >>= 1;
    }
    println!("{}", ans);
}
