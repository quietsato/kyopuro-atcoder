use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };
    let mut current = 0;
    let mut ans = 0;
    for a in a {
        current += a;
        ans = ans.min(current);
    }
    println!("{}", -ans+current);
}
