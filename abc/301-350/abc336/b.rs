use proconio::input;

fn main() {
    input! {
        x: u64
    };
    let ans = format!("{x:b}")
        .chars()
        .rev()
        .take_while(|&b| b == '0')
        .count();
    println!("{ans}");
}
