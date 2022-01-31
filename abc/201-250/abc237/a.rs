use proconio::input;

fn main() {
    input! {
        n: i64
    }

    if (n as i32) as i64 == n {
        println!("Yes");
    } else {
        println!("No");
    }
}
