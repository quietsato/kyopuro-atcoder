use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut con = vec![vec![]; n];
    let mut indeg = vec![0; n];

    (0..m).for_each(|_| {
        input! {k: usize, a: [usize; k]}
        a.iter().fold1(|current, next| {
            con[*current - 1].push(*next - 1);
            indeg[*next - 1] += 1;
            next
        });
    });

    let ans = is_dag(n, &con, &mut indeg);

    println!("{}", if ans { "Yes" } else { "No" });
}

fn is_dag(n: usize, con: &Vec<Vec<usize>>, indeg: &mut Vec<i64>) -> bool {
    let mut nexts = indeg
        .iter()
        .enumerate()
        .filter_map(|(v, &indeg)| if indeg == 0 { Some(v) } else { None })
        .collect_vec();

    let mut sorted = Vec::new();

    while let Some(current) = nexts.pop() {
        sorted.push(current);
        for &next in &con[current] {
            indeg[next] -= 1;
            if indeg[next] == 0 {
                nexts.push(next);
            }
        }
    }

    sorted.len() == n
}
