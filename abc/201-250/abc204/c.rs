use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m]
    }

    let mut adj = vec![vec![]; n];
    for (a, b) in ab {
        adj[a - 1].push(b - 1);
    }

    let mut ans = 0;

    for orig in 0..n {
        let mut dest = vec![false; n];
        dfs(orig, &mut dest, &adj);
        ans += dest.iter().filter(|b| **b).count();
    }

    println!("{}", ans);

    Ok(())
}

fn dfs(orig: usize, dest: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
    if dest[orig] {
        return;
    }
    dest[orig] = true;
    for next in &adj[orig] {
        dfs(*next, dest, adj);
    }
}
