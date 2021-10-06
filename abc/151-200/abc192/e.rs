use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

const INF: usize = 1 << 40;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
        abtk: [(Usize1, Usize1, usize, usize); m]
    }

    let mut con = vec![vec![]; n];
    for &(a, b, t, k) in &abtk {
        con[a].push((b, t, k));
        con[b].push((a, t, k));
    }

    let mut dist = vec![INF; n];
    let mut visited = vec![false; n];
    dist[x] = 0;
    let mut nexts = BinaryHeap::new();
    nexts.push(Reverse((0, x)));
    while let Some(Reverse((cost, current))) = nexts.pop() {
        if visited[current] {
            continue;
        }
        visited[current] = true;
        dist[current] = cost;
        for &(next, t, k) in &con[current] {
            if visited[next] {
                continue;
            }
            let c = cost + t + ((k - cost % k) % k);
            nexts.push(Reverse((c, next)));
        }
    }

    let ans = dist[y];
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
