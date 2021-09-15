use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let ans = {
        let mut memo = vec![vec![vec![None; 101]; 101]; 101];

        for x in (a..=100).rev() {
            for y in (b..=100).rev() {
                for z in (c..=100).rev() {
                    dp(&mut memo, x, y, z);
                }
            }
        }

        memo[a][b][c].unwrap()
    };

    println!("{}", ans);
}

fn dp(memo: &mut Vec<Vec<Vec<Option<f64>>>>, x: usize, y: usize, z: usize) -> f64 {
    if x == 100 || y == 100 || z == 100 {
        return 0.0;
    }

    return memo[x][y][z].unwrap_or_else(|| {
        let v = (x as f64) / ((x + y + z) as f64) * (dp(memo, x + 1, y, z) + 1.0)
            + (y as f64) / ((x + y + z) as f64) * (dp(memo, x, y + 1, z) + 1.0)
            + (z as f64) / ((x + y + z) as f64) * (dp(memo, x, y, z + 1) + 1.0);
        memo[x][y][z] = Some(v);
        v
    });
}
