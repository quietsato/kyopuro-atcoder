use proconio::input;

fn main() {
    input! {
        k: usize,
        s: String
    }

    println!(
        "{}",
        if s.len() <= k {
            s
        } else {
            s.chars().take(k).chain("...".chars()).collect()
        }
    )
}
