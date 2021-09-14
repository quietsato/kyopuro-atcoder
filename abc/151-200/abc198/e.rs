use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        ab: [(Usize1, Usize1); n-1]
    }

    let ans = {
        let con = {
            let mut con = vec![Vec::new(); n];
            for (a, b) in ab {
                con[a].push(b);
                con[b].push(a);
            }
            con
        };
        let mut count_per_color = vec![0; 10usize.pow(5)];
        let mut visited = vec![false; n];
        let mut ok = vec![false; n];

        dfs(0, &mut count_per_color, &mut visited, &mut ok, &con, &c);

        (1u32..)
            .zip(ok.iter())
            .filter(|(_, b)| **b)
            .map(|(i, _)| i.to_string())
            .join("\n")
    };

    println!("{}", ans);
}

fn dfs(
    current: usize,
    count_per_color: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    ok: &mut Vec<bool>,
    con: &Vec<Vec<usize>>,
    colors: &Vec<usize>,
) {
    visited[current] = true;

    let c = colors[current];
    count_per_color[c] += 1;

    if count_per_color[c] == 1 {
        ok[current] = true;
    }

    for &next in &con[current] {
        if !visited[next] {
            dfs(next, count_per_color, visited, ok, con, colors);
        }
    }

    count_per_color[c] -= 1;
}
