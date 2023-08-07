use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
    };
    println!(
        "{}",
        if (r as i64 - 8).abs().max((c as i64 - 8).abs()) as u64 & 1 == 1 {
            "black"
        } else {
            "white"
        }
    );
}
