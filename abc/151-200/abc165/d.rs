use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        n: u64
    }

    println!("{}", (f(a, b, (b-1).min(n))).max(f(a, b, n)));

    // 5 7 20
    // let v = (1u64..=n).map(|i| f(a, b, i)).collect_vec();
    // println!("{:?}", v);
    // -> [0, 1, 2, 2, 3, 4, 0, 0, 1, 2, 2, 3, 4, 0, 0, 1, 2, 2, 3, 4]
}

fn f(a: u64, b: u64, x: u64) -> u64 {
    (a * x) / b - a * (x / b)
}
