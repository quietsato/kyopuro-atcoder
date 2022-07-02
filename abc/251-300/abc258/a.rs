use proconio::input;

fn main() {
    input! {
        k:  u64
    }

    println!("{:02}:{:02}", 21 + (k / 60), k % 60,);
}
