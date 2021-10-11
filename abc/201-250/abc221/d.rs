use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, usize); n]
    }

    let mut event = vec![];

    for &(a, b) in &ab {
        event.push((a, 1));
        event.push((a + b, -1));
    }

    event.sort_by(|x, y| y.0.cmp(&x.0));

    // for ev in &event {
    //     println!("{:?}", &ev);
    // }

    let mut cnt = vec![0; n + 1];
    let mut now_day = 0usize;
    let mut now_login = 0i64;

    while let Some((day, ev)) = event.pop() {
        if now_day < day {
            cnt[now_login as usize] += day - now_day;
        }
        now_login += ev;
        now_day = day;
        // println!("{:?}", &cnt);
    }

    println!("{}", cnt[1..].iter().join(" "));
}
