use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        ck: [ (char, usize); k ]
    }

    const MOD: u64 = 998244353;

    let mut ans = 1;

    for i in 1..=n {
        let mut a = k;
        for &(c, ki) in &ck {
            match c {
                'L' if i < ki => a -= 1,
                'R' if i > ki => a -= 1,
                _ if i == ki => a = 1,
                _ => {}
            }
            if a == 1 {
                break;
            }
        }
        ans *= a;
        ans %= MOD;
    }

    println!("{}", ans);
}
