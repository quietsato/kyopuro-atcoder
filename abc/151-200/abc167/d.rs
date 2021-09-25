use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [Usize1; n]
    }

    let mut edge = vec![n + 1; n];
    let mut visited_order = vec![0usize; n];

    let mut cycle_len = 0;
    let mut cycle_start = 0;
    let mut current = 0;
    loop {
        let next = a[current];

        edge[current] = next;
        if edge[next] != n + 1 {
            cycle_len = visited_order[current] - visited_order[next] + 1;
            cycle_start = next;
            break;
        }
        visited_order[next] = visited_order[current] + 1;

        current = next;

        k -= 1;
        if k == 0 {
            break;
        }
    }

    if k == 0 {
        println!("{}", current + 1);
        return;
    }

    let mut ans = cycle_start;
    for _ in 0..((k - 1) % cycle_len) {
        ans = edge[ans];
    }
    println!("{}", ans + 1);
}
