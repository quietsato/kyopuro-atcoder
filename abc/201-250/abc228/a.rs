use proconio::input;

fn main() {
    input! {
        s: i64,
        mut t: i64,
        x: i64
    }

    if t < s {
        t += 24
    }

    if (s <= x && x < t) || (s <= (x + 24) && (x + 24) < t) {
        println!("Yes")
    } else {
        println!("No")
    }
}
