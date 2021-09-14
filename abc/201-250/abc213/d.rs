use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize);n-1]
    }

    let mut edges = vec![vec![]; n];
    ab.iter().for_each(|(a, b)| {
        edges[*a - 1].push(*b - 1);
        edges[*b - 1].push(*a - 1);
    });
    edges.iter_mut().for_each(|v| {
        v.sort();
    });

    let mut visited = vec![false; n];
    let mut course = vec![];

    dfs(&mut visited, &mut course, &edges, 0);

    println!("{}", course.iter().map(|v| v.to_string()).join(" "));
}

fn dfs(visited: &mut Vec<bool>, course: &mut Vec<usize>, edges: &Vec<Vec<usize>>, next: usize) {
    visited[next] = true;
    course.push(next + 1);

    for e in &edges[next] {
        if !visited[*e] {
            dfs(visited, course, edges, *e);
            visited[next] = true;
            course.push(next + 1);
        }
    }
}
