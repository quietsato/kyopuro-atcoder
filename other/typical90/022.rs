use proconio::input;

// 022 Cubic Cake (★2)
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    let g = gcd(gcd(a, b), c);

    println!("{}", a / g - 1 + b / g - 1 + c / g - 1);
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a.max(b), a.min(b));

    while b > 0 {
        let c = a % b;
        a = std::mem::replace(&mut b, c);
    }

    a
}
