use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    println!(
        "{}",
        match s.cmp(&t) {
            std::cmp::Ordering::Less => "Yes",
            _ => "No",
        }
    );
}
