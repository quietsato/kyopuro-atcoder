use proconio::input;

fn main() {
    input! {
        s: String
    }
    if "oxx".repeat(10).contains(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}

