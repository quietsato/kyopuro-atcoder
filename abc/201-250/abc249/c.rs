use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n]
    }

    let mut count = vec![0; 26];
    let mut ans = 0;
    for bit in 0u32..(1 << n) {
        for item in count.iter_mut() {
            *item = 0;
        }
        for (i, s) in s.iter().enumerate().take(n) {
            if (bit >> i) & 1 == 0 {
                continue;
            }
            for c in s.iter() {
                count[(*c as u8 - b'a') as usize] += 1;
            }
        }
        ans = ans.max(count.iter().filter(|cnt| **cnt == k).count());
    }
    println!("{}", ans);
}
