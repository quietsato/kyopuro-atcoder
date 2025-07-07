use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; q]
    };

    let mut cell = vec![false; n];

    let mut ans = 0;

    for a in a {
        cell[a] = !cell[a];

        ans += if n == 1 {
            // N=1 の場合は特殊な独立した分岐にする
            if cell[a] {
                1
            } else {
                -1
            }
        } else if a == 0 {
            // 左端
            if cell[a + 1] {
                // 右が黒
                0
            } else {
                // 右が白
                if cell[a] {
                    // 黒に塗る
                    1
                } else {
                    // 白に塗る
                    -1
                }
            }
        } else if a == n - 1 {
            // 右端
            if cell[a - 1] {
                // 左が黒
                0
            } else {
                // 左が白
                if cell[a] {
                    // 黒に塗る
                    1
                } else {
                    // 白に塗る
                    -1
                }
            }
        } else {
            // 左端・右端を除く
            if cell[a - 1] != cell[a + 1] {
                // 左・右の色が異なる
                0
            } else if cell[a - 1] {
                // 両隣が黒
                if cell[a] {
                    // 黒に塗る
                    -1
                } else {
                    // 白に塗る
                    1
                }
            } else {
                // 両隣が白
                if cell[a] {
                    // 黒に塗る
                    1
                } else {
                    // 白に塗る
                    -1
                }
            }
        };

        println!("{}", ans);
    }
}
