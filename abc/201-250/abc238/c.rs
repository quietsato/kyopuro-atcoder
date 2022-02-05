use proconio::input;

const MOD: u128 = 998244353;

fn main() {
    input! {
        n: u64
    }

    let mut ans: u128 = 0;

    let num = n.num_of_digits();

    for i in 1..num {
        let x = (9 * 10u64.pow((i - 1) as u32)) as u128;
        ans += ((x * (1 + x)) / 2) % MOD;
        ans %= MOD;
    }

    {
        let x = (n - 10u64.pow((num - 1) as u32) + 1) as u128;
        ans += ((x * (1 + x)) / 2) % MOD;
    }

    println!("{}", ans % MOD);
}

trait CountNumOfDigits {
    fn num_of_digits(&self) -> usize;
}
impl CountNumOfDigits for u64 {
    fn num_of_digits(&self) -> usize {
        let mut n = 1;
        let mut x = *self;
        loop {
            x /= 10;
            if x == 0 {
                break;
            }
            n += 1;
        }
        n
    }
}
