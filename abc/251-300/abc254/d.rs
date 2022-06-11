use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut ans = 0;

    for i in 1..=n {
        let k = {
            let mut k = i;
            for d in (2..=i).take_while(|d| d * d <= i) {
                while k % (d * d) == 0 {
                    k /= d * d;
                }
            }
            k
        };
        for d in 1..=n {
            let j = k * d * d;
            if j <= n {
                ans += 1;
            } else {
                break;
            }
        }
    }

    println!("{}", ans);
}
