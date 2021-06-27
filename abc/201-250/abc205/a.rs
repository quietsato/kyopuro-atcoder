use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        a: f64,
        b: f64
    }

    println!("{}", (b / 100.0) * a);

    Ok(())
}
