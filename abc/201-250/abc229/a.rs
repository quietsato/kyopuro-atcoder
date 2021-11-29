use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    if s1 == "#." && s2 == ".#" {
        println!("No");
    } else if s1 == ".#" && s2 == "#." {
        println!("No");
    } else {
        println!("Yes");
    }
}
