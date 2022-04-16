use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        q: usize
    }

    let mut arr = vec![vec![]; n];

    for (i, a) in a.into_iter().enumerate() {
        arr[a].push(i);
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
            x: Usize1
        }
        let li = arr[x].binary_search(&l).map_or_else(|e| e, |v| v);
        let ri = arr[x].binary_search(&(r + 1)).map_or_else(|e| e, |v| v);
        println!("{}", ri - li);
    }
}
