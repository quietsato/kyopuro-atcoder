use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize
    }

    let mut ans = 0;
    ans += if r - 1 > 0 { 1 } else { 0 };
    ans += if c - 1 > 0 { 1 } else { 0 };
    ans += if r < h { 1 } else { 0 };
    ans += if c < w { 1 } else { 0 };
    println!("{}", ans);
}
