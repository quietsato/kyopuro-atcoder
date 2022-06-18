use proconio::input;

fn main() {
    input! {
        h: [i64; 3],
        w: [i64; 3],
    }

    let mut ans = 0;
    for a11 in 1..=28 {
        for a12 in 1..=28 {
            for a21 in 1..=28 {
                for a22 in 1..=28 {
                    let a13 = h[0] - a11 - a12;
                    let a23 = h[1] - a21 - a22;
                    let a31 = w[0] - a11 - a21;
                    let a32 = w[1] - a12 - a22;

                    if a13 <= 0 || a23 <= 0 || a31 <= 0 || a32 <= 0 {
                        continue;
                    }

                    let a33_h = h[2] - a31 - a32;
                    let a33_w = w[2] - a13 - a23;

                    if a33_h != a33_w || a33_h <= 0 {
                        continue;
                    }
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
