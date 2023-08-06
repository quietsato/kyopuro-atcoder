use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut ans = vec![];
    for bit in 0..(1 << m) {
        let cnt = (0..m).filter(|i| (bit >> i) & 1 == 1).count();
        if cnt != n {
            continue;
        }
        // println!("{:05b}", bit);
        ans.push(
            (0..m)
                .filter_map(|i| {
                    if (bit >> i) & 1 == 1 {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect_vec(),
        );
    }

    ans.sort();
    println!("{}", ans.iter().map(|a| a.iter().join(" ")).join("\n"));
}
