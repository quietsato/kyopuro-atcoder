use proconio::input;

fn main() {
    const DIV: i64 = 998244353;
    input! {
        n: i64
    };
    let a = DIV * (n / DIV);
    println!("{}", n - a + if a > n { DIV } else { 0 });
}
