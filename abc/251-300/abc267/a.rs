use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String
    };
    const WEEKDAYS: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    println!(
        "{}",
        WEEKDAYS
            .iter()
            .find_position(|&w| w == &s)
            .map(|(i, _)| 5 - i)
            .unwrap()
    );
}
