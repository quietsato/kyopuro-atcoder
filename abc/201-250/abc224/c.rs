use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let v1 = (xy[i].0 - xy[j].0, xy[i].1 - xy[j].1);
                let v2 = (xy[i].0 - xy[k].0, xy[i].1 - xy[k].1);
                if (v1.0 * v2.1 - v2.0 * v1.1).abs() > 0 {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
