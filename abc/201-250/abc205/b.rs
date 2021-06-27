use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
        mut a: [i64; n]
    }

    a.sort();

    if a.iter().zip(1..).map(|(a, b)| *a == b).all(|b| b) {
        println!("Yes");
    } else {
        println!("No");
    }

    Ok(())
}
