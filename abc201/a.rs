use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        mut a: [i32; 3]
    }
    a.sort();

    if a[1] - a[0] == a[2] - a[1] {
        println!("Yes");
    } else {
        println!("No");
    }

    Ok(())
}

