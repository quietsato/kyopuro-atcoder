use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
        f: u64,
        x: u64,
    }

    let mut takahashi = 0;
    let mut aoki = 0;

    for t in 0..x {
        if t % (a + c) < a {
            takahashi += b;
        }
        if t % (d + f) < d {
            aoki += e;
        }
    }

    match takahashi.cmp(&aoki) {
        std::cmp::Ordering::Less => println!("Aoki"),
        std::cmp::Ordering::Equal => println!("Draw"),
        std::cmp::Ordering::Greater => println!("Takahashi"),
    }
}
