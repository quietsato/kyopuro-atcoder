use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(u64, u64); n]
    };
    let x = xy.iter().map(|(x, _)| x).sum::<u64>();
    let y = xy.iter().map(|(_, y)| y).sum::<u64>();
    let ans = match x.cmp(&y) {
        std::cmp::Ordering::Less => "Aoki",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Takahashi",
    };
    println!("{ans}");
}
