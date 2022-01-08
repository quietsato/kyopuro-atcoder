use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
    }

    let mut bags = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            a: [u64; l]
        }
        bags.push(a);
    }

    println!("{}", dfs(&bags, 0, n, x, 1));
}

fn dfs(bags: &Vec<Vec<u64>>, i: usize, n: usize, x: u64, mul: u64) -> u64 {
    if i >= n {
        if mul == x {
            return 1;
        } else {
            return 0;
        }
    }
    let mut ans = 0;
    for item in bags[i].iter() {
        if x / mul < *item {
            continue;
        }
        ans += dfs(bags, i + 1, n, x, mul * item);
    }
    return ans;
}
