use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        a: i64,
        b: i64,
        c: u64
    }

    if a == b || c == 1 {
        println!("=");
    } else if c & 1 == 0 {
        // 偶数乗
        if a.abs() == b.abs() {
            println!("=");
        } else if a.abs() > b.abs() {
            println!(">");
        } else {
            println!("<");
        }
    } else {
        // 奇数乗
        if a < 0 && b >= 0 {
            println!("<");
        } else if a >= 0 && b < 0 {
            println!(">");
        } else {
            if a > b {
                println!(">");
            } else {
                println!("<");
            }
        }
    }

    Ok(())
}
