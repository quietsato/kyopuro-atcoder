use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    println!("{}", if (c <= b && b <= a) || (a <= b && b <= c) {"Yes"} else {"No"});
}
