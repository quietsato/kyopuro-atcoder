use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ans = vec![vec!["T".into(); n]; n];

    let mut i = 0i32;
    let mut j = 0i32;

    let mut di = 1i32;
    let mut dj = 0i32;

    for x in 1..n.pow(2) {
        ans[j as usize][i as usize] = x.to_string();
        if !(0..n as i32).contains(&(i + di))
            || !(0..n as i32).contains(&(j + dj))
            || ans[(j + dj) as usize][(i + di) as usize] != "T"
        {
            (di, dj) = (-dj, di);
        }
        i += di;
        j += dj;
    }
    for a in ans {
        println!("{}", a.join(" "));
    }
}
