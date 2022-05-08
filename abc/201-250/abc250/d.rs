use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut is_prime = vec![true; 1_000_001];
    'outer: for i in 3..=1_000_000 {
        if !is_prime[i] {
            continue;
        }
        for p in 2..=(i as f32).sqrt() as usize {
            if !is_prime[p] {
                continue;
            }
            if i % p == 0 {
                is_prime[i] = false;
                for j in (i..=1_000_000).step_by(i) {
                    is_prime[j] = false;
                }
                continue 'outer;
            }
        }
    }
    let mut primes = vec![];
    (2..=1_000_000).for_each(|p| {
        if is_prime[p] {
            primes.push(p);
        }
    });

    let mut ans = 0;
    for i in 0..primes.len() {
        for j in 0..i {
            let k = primes[j] * primes[i] * primes[i] * primes[i];
            if k <= n {
                ans += 1;
            } else {
                break;
            }
        }
    }

    println!("{}", ans);
}
