use proconio::input;

fn main() {
    input! {
        h: u32,
        w: u32,
        a: u32,
        b: u32
    }

    let mut ans = 0;
    dfs(h, w, a, b, 0, 0, &mut ans);
    println!("{}", ans);
}

fn dfs(h: u32, w: u32, a: u32, b: u32, i: u32, bit: u32, ans: &mut u32) {
    if i == h * w {
        *ans += 1;
        return;
    }

    if bit & (1 << i) > 0 {
        dfs(h, w, a, b, i + 1, bit, ans);
    }

    if b > 0 {
        dfs(h, w, a, b - 1, i + 1, bit | (1 << i), ans);
    }

    if a > 0 {
        if i % w != w - 1 && bit & (1 << (i + 1)) == 0 {
            dfs(h, w, a - 1, b, i + 1, bit | (0b11 << i), ans);
        }
        if (i + w) < h * w && bit & (1 << (i + w)) == 0 {
            dfs(h, w, a - 1, b, i + 1, bit | (1 << i) | (1 << (i + w)), ans);
        }
    }
}
