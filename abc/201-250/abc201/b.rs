use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
        mut m: [(String, i64); n]
    }

    m.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{}", m[1].0);

    Ok(())
}

