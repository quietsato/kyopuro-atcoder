use proconio::input;

fn main() {
    input! {
        k: u32,
        a: String,
        b: String
    }

    let a = u64::from_str_radix(&a, k).unwrap();
    let b = u64::from_str_radix(&b, k).unwrap();

    println!("{}", a * b);
}
