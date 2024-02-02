use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    };
    let mut ans = k + 1;
    // 横方向
    for s in &s {
        if k > w {
            break;
        }
        let mut cnt_x = 0;
        let mut cnt_o = 0;
        // 左端k個
        for i in 0..k {
            match s[i] {
                'x' => {
                    cnt_x += 1;
                }
                'o' => {
                    cnt_o += 1;
                }
                _ => {},
            }
        }
        if cnt_x == 0 {
            ans = ans.min(k - cnt_o);
        }

        // それより右は差分
        for i in k..w {
            match s[i - k] {
                'x' => {
                    cnt_x -= 1;
                }
                'o' => {
                    cnt_o -= 1;
                }
                _ => {},
            }
            match s[i] {
                'x' => {
                    cnt_x += 1;
                }
                'o' => {
                    cnt_o += 1;
                }
                _ => {},
            }
            if cnt_x == 0 {
                ans = ans.min(k - cnt_o);
            }
        }
    }

    // 縦方向
    for j in 0..w {
        let s = s.iter().map(|s| s[j].clone()).collect_vec();

        if k > h {
            break;
        }
        let mut cnt_x = 0;
        let mut cnt_o = 0;
        // 上端k個
        for i in 0..k {
            match s[i] {
                'x' => {
                    cnt_x += 1;
                }
                'o' => {
                    cnt_o += 1;
                }
                _ => {},
            }
        }
        if cnt_x == 0 {
            ans = ans.min(k - cnt_o);
        }

        // それより下は差分
        for i in k..h {
            match s[i - k] {
                'x' => {
                    cnt_x -= 1;
                }
                'o' => {
                    cnt_o -= 1;
                }
                _ => {},
            }
            match s[i] {
                'x' => {
                    cnt_x += 1;
                }
                'o' => {
                    cnt_o += 1;
                }
                _ => {},
            }
            if cnt_x == 0 {
                ans = ans.min(k - cnt_o);
            }
        }
    }

    println!(
        "{}",
        (ans == k + 1).then(|| "-1").unwrap_or(&ans.to_string())
    );
}
