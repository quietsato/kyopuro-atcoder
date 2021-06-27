use proconio::input;

fn main(){
    input! {
        n: f64
    }

    match ((n * 1.08).floor() as u64).cmp(&206) {
        std::cmp::Ordering::Less => println!("Yay!"),
        std::cmp::Ordering::Equal => println!("so-so"),
        std::cmp::Ordering::Greater => println!(":("),
    }
}
