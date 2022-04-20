use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        xy: [(i64, i64); n]
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut checked = vec![vec![false; n]; n];
    let mut ans = 0;
    let mut list = vec![];

    for i in 0..n {
        for j in i + 1..n {
            if checked[i][j] {
                continue;
            }

            let mut count = 2;
            list.clear();
            list.push(i);
            list.push(j);

            for l in j + 1..n {
                let (x0, y0) = xy[i];
                let (x1, y1) = xy[j];
                let (x, y) = xy[l];
                let colinear = (x1 - x0) * (y - y0) == (y1 - y0) * (x - x0);
                if colinear {
                    count += 1;
                    list.push(l);
                }
            }

            for (ii, &v) in list.iter().enumerate() {
                for &w in list.iter().skip(ii + 1) {
                    checked[v][w] = true;
                }
            }

            if count >= k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
