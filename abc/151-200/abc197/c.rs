#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

/*
    bit全探索

    a. n = 3 のとき，bitsの範囲は0b00以上0b11以下 (1 << 2 が 0b100 であることから)
    b. 区間ごとのORをとる
    c. 最終要素を配列添字iが越えた時，もしくは下位iビット目が1(区間分割)のとき，次の操作を行う．
        c-1. 複数区間で求めたORどうしについて，XORをとる
        c-2. 新たな区間を作成する(OR結果の初期化)
*/

fn main() {
    let n: u32 = parse_line().unwrap();
    let a: Vec<u64> = parse_line().unwrap();

    let ans = {
        let mut ans = u64::MAX;

        for bits in 0..(1 << n - 1) {
            // a.
            let (mut or, mut xor) = (0, 0);

            for i in 0..=n as usize {
                if i < n as usize {
                    or = or | a[i]; // b.
                }
                if i == n as usize || (bits >> i) & 1 == 1 {
                    // c.
                    xor = xor ^ or; // c-1.
                    or = 0; // c-2.
                }
            }

            ans = ans.min(xor);
        }

        ans
    };

    println!("{}", ans);
}
