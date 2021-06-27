use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        a: i64,
        (b, c): (i64, i64),
        s: String
    }

    println!("{} {}", a + b + c, s);

    Ok(())
}
