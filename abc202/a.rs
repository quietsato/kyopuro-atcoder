use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        x: [i64; 3]
    }

    println!("{}", x.iter().map(|n| { 7 - n }).sum::<i64>());

    Ok(())
}

