use proconio::input;

fn main() {
    input! {
        t: usize,
        pq: [(u64, u64); t]
    }

    for (a, s) in pq {
        let ok = (a << 1) <= s && s >= a && ((s - a) & a == a);
        YesNo(ok);
    }
}

#[allow(non_snake_case)]
fn yesno(b: bool) {
    println!("{}", if b { "yes" } else { "no" });
}
#[allow(non_snake_case)]
fn YesNo(b: bool) {
    println!("{}", if b { "Yes" } else { "No" });
}
#[allow(non_snake_case)]
fn YESNO(b: bool) {
    println!("{}", if b { "YES" } else { "NO" });
}
