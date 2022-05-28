use proconio::input;

fn main() {
    input! {
        n: u128,
        a: u128,
        b: u128,
    }

    let all = n * (n + 1) / 2;
    let mula = (n / a) * (a + (n / a * a)) / 2;
    let mulb = (n / b) * (b + (n / b * b)) / 2;
    let lcm = lcm(a, b);
    let mulab = (n / lcm) * (lcm + (n / lcm * lcm)) / 2;

    println!("{}", all + mulab - mula - mulb);
}

fn lcm(a: u128, b: u128) -> u128 {
    if a == 1 || b == 1 {
        return a.max(b);
    }
    let mut a = a;
    let mut b = b;
    let mut ret = 1;

    let mut div = 2;
    loop {
        if a == 1 && b == 1 {
            return ret;
        }
        if a % div == 0 && b % div == 0 {
            a /= div;
            b /= div;
            ret *= div;
        } else if a % div != 0 && b % div != 0 {
            div += 1;
        } else {
            ret *= a * b;
            return ret;
        }
    }
}
