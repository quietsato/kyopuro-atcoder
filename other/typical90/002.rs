use itertools::Itertools;
use proconio::input;

// 002 Encyclopedia of Parentheses（★3）
fn main() {
    input! {
        n: usize
    }
    if n & 1 == 1 {
        return;
    }

    dfs(n / 2, n / 2, 0, n, 0);
}

fn dfs(rem_open: usize, rem_close: usize, pos: usize, n: usize, bits: u32) {
    if rem_open == 0 && rem_close == 0 {
        println!(
            "{}",
            (0..n)
                .map(|i| {
                    if is_nth_bit_one(bits, i) {
                        '('
                    } else {
                        ')'
                    }
                })
                .join("")
        );
    } else {
        if rem_open > 0 {
            dfs(rem_open - 1, rem_close, pos + 1, n, bits | (1 << pos));
        }
        if rem_close > rem_open {
            dfs(rem_open, rem_close - 1, pos + 1, n, bits)
        }
    }
}

use num_traits::{PrimInt, Unsigned};
fn is_nth_bit_one<T: PrimInt + Unsigned>(bits: T, n: usize) -> bool {
    (bits >> n) & T::one() == T::one()
}
