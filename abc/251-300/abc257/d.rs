use proconio::input;

fn main() {
    input! {
        n: usize,
        xyp: [(i64, i64, u64); n],
    }
    const INF: u64 = std::u64::MAX >> 1; // INF + INF == std::u64::MAX - 1

    let mut dist = vec![vec![INF; n]; n];
    for (i, (x1, y1, p)) in xyp.iter().enumerate() {
        for (j, (x2, y2, _)) in xyp.iter().enumerate() {
            if i == j {
                dist[i][j] = 0;
            } else {
                dist[i][j] = (((x1 - x2).abs() as u64 + (y1 - y2).abs() as u64) + p - 1) / p;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let current = dist[i][j];
                let new = dist[i][k].max(dist[k][j]);
                if current > new {
                    dist[i][j] = new;
                }
            }
        }
    }

    println!(
        "{}",
        dist.iter().flat_map(|v| v.iter().max()).min().unwrap()
    );
}
