use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        ab: [(u64, u64); n]
    }

    let mut ans = vec![];
    let mut time = 0;
    for (i, (a, b)) in ab.iter().enumerate() {
        time += a + b;
        ans.push(time + (x - i as u64 - 1) * b);
        if x == i as u64 + 1 {
            break;
        }
    }

    println!("{}", ans.iter().min().unwrap());
}
