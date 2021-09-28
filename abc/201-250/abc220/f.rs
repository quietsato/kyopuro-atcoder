use proconio::{input, marker::Usize1};

const INF: usize = 1_000_000;

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1]
    }

    let con = {
        let mut con = vec![vec![]; n];
        for &(u, v) in &uv {
            con[u].push(v);
            con[v].push(u);
        }
        con
    };

    let (d, sub) = {
        let mut d = vec![INF; n];
        let mut sub = vec![1usize; n];
        d[0] = 0;

        dfs(0, &con, &mut d, &mut sub);

        (d, sub)
    };

    let ans = {
        let mut ans = vec![0; n];
        ans[0] = d.iter().sum();

        dfs2(0, &con, &mut ans, &d, &sub, n);

        ans
    };

    for a in ans {
        println!("{}", a);
    }
}

fn dfs(current: usize, con: &Vec<Vec<usize>>, d: &mut Vec<usize>, sub: &mut Vec<usize>) {
    for &next in &con[current] {
        if d[next] != INF {
            continue;
        }
        d[next] = d[current] + 1;
        dfs(next, con, d, sub);
        sub[current] += sub[next];
    }
}

fn dfs2(
    current: usize,
    con: &Vec<Vec<usize>>,
    ans: &mut Vec<usize>,
    d: &Vec<usize>,
    sub: &Vec<usize>,
    n: usize,
) {
    for &next in &con[current] {
        if ans[next] != 0 {
            continue;
        }
        ans[next] = ans[current] + n - 2 * sub[next];
        dfs2(next, con, ans, d, sub, n);
    }
}
