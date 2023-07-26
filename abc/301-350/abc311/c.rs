use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut visited: Vec<Option<usize>> = vec![None; n];

    for i in 0..n {
        if visited[i].is_some() {
            continue;
        }
        let mut v = i;
        loop {
            let next = a[v];
            if visited[next].is_some() {
                visited[next] = Some(v);

                // found loop
                let mut ans = vec![];
                let end = v;
                loop {
                    ans.push(v);
                    v = a[v];
                    if v == end {
                        println!("{}", ans.len());
                        println!("{}", ans.iter().map(|v| v + 1).join(" "));
                        return;
                    }
                }
            }
            visited[next] = Some(v);
            v = next;
        }

        // println!("{:?}", &visited);
    }
}
