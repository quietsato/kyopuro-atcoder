use num_traits::PrimInt;
use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[u32; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[u32; w2]; h2],
    };

    for h in 0..(1 << h1) {
        if h.count_ones() != h2 as u32 {
            continue;
        }
        for w in 0..(1 << w1) {
            if w.count_ones() != w2 as u32 {
                continue;
            }
            let mut flag = true;

            for (ib, ia) in (0..h1).filter(|i| (h >> i) & 1 == 1).enumerate() {
                for (jb, ja) in (0..w1).filter(|j| (w >> j) & 1 == 1).enumerate() {
                    flag &= a[ia][ja] == b[ib][jb];
                }
            }

            if flag {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
