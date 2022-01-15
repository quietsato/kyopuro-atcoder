use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let a = s.iter().collect::<String>().parse::<i32>().unwrap();
    let b = format!("{}{}{}", s[1], s[2], s[0]).parse::<i32>().unwrap();
    let c = format!("{}{}{}", s[2], s[0], s[1]).parse::<i32>().unwrap();

    println!("{}", a + b + c);
}
