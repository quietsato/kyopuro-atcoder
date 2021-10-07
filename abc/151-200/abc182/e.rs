use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
        cd: [(Usize1, Usize1); m],
    }

    let mut light_h = vec![vec![0; w]; h];
    let mut light_v = vec![vec![0; w]; h];
    for &(c, d) in &cd {
        light_h[c][d] = -1;
        light_v[c][d] = -1;
    }

    for &(a, b) in &ab {
        // Horizontal
        if light_h[a][b] == 0 {
            for x in (0..=b).rev() {
                if light_h[a][x] == -1 {
                    break;
                }
                light_h[a][x] = 1;
            }
            for x in b..w {
                if light_h[a][x] == -1 {
                    break;
                }
                light_h[a][x] = 1;
            }
        }
        // Vertical
        if light_v[a][b] == 0 {
            for y in (0..=a).rev() {
                if light_v[y][b] == -1 {
                    break;
                }
                light_v[y][b] = 1;
            }
            for y in a..h {
                if light_v[y][b] == -1 {
                    break;
                }
                light_v[y][b] = 1;
            }
        }
    }

    let mut ans = 0usize;
    for y in 0..h {
        for x in 0..w {
            if light_v[y][x] == 1 || light_h[y][x] == 1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
