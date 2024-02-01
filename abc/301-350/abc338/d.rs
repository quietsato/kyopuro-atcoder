use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
    };

    // 橋iを渡るとき
    let mut idxs = vec![vec![]; n];
    for (i, &x) in x.iter().enumerate() {
        idxs[x].push(i);
    }

    // 橋Nを封鎖した場合
    let mut diff = Vec::with_capacity(m);
    for (&x0, &x1) in x.iter().zip(x.iter().skip(1)) {
        diff.push(x0.abs_diff(x1));
    }
    let mut ans = diff.iter().sum::<usize>();
    let mut a = ans;

    // それぞれの橋を封鎖した場合
    for idxs in idxs {
        for &i in &idxs {
            // 始点以外
            if i > 0 {
                let prev = diff[i - 1];
                a = a - prev + (n - prev);
                diff[i - 1] = n - prev;
            }
            // 終点以外
            if i < m - 1 {
                let prev = diff[i];
                a = a - prev + (n - prev);
                diff[i] = n - prev;
            }
        }
        ans = ans.min(a);
    }
    println!("{}", ans);
}
