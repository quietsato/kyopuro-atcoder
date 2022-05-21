use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut ans = vec![0; 10];

    for target in 0..=9 {
        let target_c = std::char::from_digit(target, 10).unwrap();
        let idx = s
            .iter()
            .map(|s| s.iter().find_position(|&&c| c == target_c).unwrap().0)
            .collect_vec();
        let mut count = vec![0; 10];
        for i in idx {
            count[i] += 1;
        }
        for (idx, &count) in count.iter().enumerate() {
            for x in 0..count {
                ans[target as usize] = ans[target as usize].max(10 * x + idx);
            }
        }
    }

    println!("{}", ans.iter().min().unwrap());
}
