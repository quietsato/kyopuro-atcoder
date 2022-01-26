use std::collections::{BTreeMap, BTreeSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        a: i64,
        n: i64
    }

    let mut adj = &mut BTreeMap::new();
    construct(a, 1, &mut adj);

    println!("{}", bfs(adj, n));
}

fn bfs(adj: &BTreeMap<i64, BTreeSet<i64>>, n: i64) -> i64 {
    if !adj.contains_key(&n) {
        return -1;
    }

    let mut nexts = VecDeque::new();
    let mut dist = BTreeMap::new();

    for v in adj.keys() {
        dist.insert(v, -1i64);
    }

    nexts.push_back((0, 1));

    while let Some((d, current)) = nexts.pop_front() {
        for next in adj.get(&current).unwrap() {
            let current_d = dist.get_mut(&current).unwrap();
            *current_d = d;

            let next_d = dist.get_mut(&next).unwrap();
            if *next_d == -1 || *next_d > d + 1 {
                *next_d = d + 1;
                nexts.push_back((d + 1, *next));
            }
        }
    }

    dist.get(&n).unwrap().to_owned()
}

fn construct(a: i64, x: i64, adj: &mut BTreeMap<i64, BTreeSet<i64>>) {
    if !adj.contains_key(&x) {
        adj.insert(x, BTreeSet::new());
    }

    let next_x = x * a;
    if next_x < 1_000_000 {
        let v = adj.get_mut(&x).unwrap();
        if !v.contains(&next_x) {
            v.insert(next_x);
            construct(a, next_x, adj);
        }
    }

    if x > 10 && x % 10 > 0 {
        let num_of_digits = {
            let mut x = x;
            let mut num = 1;
            loop {
                x /= 10;
                if x == 0 {
                    break;
                }
                num += 1;
            }
            num
        };
        let next_x = x / 10 + (x % 10) * 10i64.pow(num_of_digits - 1);
        let v = adj.get_mut(&x).unwrap();
        if !v.contains(&next_x) {
            v.insert(next_x);
            construct(a, next_x, adj);
        }
    }
}
