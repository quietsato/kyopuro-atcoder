use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut ans = 0;
    for a in 1.. {
        if a * a * a > n {
            break;
        }
        for b in a..=(n / a) {
            let c = n / a / b;
            if c < b {
                break;
            }
            ans += c - b + 1;
        }
    }

    println!("{}", ans);
}
