use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    if s == t {
        println!("Yes");
        return;
    }

    for i in 1..s.len() {
        let mut u = s.clone();
        u.swap(i - 1, i);

        if u == t {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
