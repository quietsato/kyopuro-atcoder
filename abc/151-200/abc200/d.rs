use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [u32; n]
    }

    let n = n.min(8);
    for a in a.iter_mut() {
        *a %= 200;
    }

    let mut m: Vec<Vec<u32>> = vec![vec![]; 200];

    for bit in 1..2u32.pow(n) {
        let mut sum = 0;
        for i in 0..(n as usize) {
            if (bit >> i) & 1 == 1 {
                sum += a[i];
            }
        }
        let sum_mod = (sum % 200) as usize;
        if !m[sum_mod].is_empty() {
            println!("Yes");
            println!(
                "{} {}",
                m[sum_mod][0].count_ones(),
                vector_to_string(m[sum_mod][0], n)
            );
            println!("{} {}", bit.count_ones(), vector_to_string(bit, n));
            return;
        }
        m[sum_mod].push(bit);
    }

    println!("No");
}

fn vector_to_string(bit: u32, n: u32) -> String {
    (0..n)
        .filter_map(|i| {
            let i = i as usize;
            if (bit >> i) & 1 == 1 {
                Some((i + 1).to_string())
            } else {
                None
            }
        })
        .join(&" ")
}
