use proconio::input;

fn prod(o: (i64, i64), a: (i64, i64), b: (i64, i64)) -> i64 {
    let t = (a.0 - o.0, a.1 - o.1);
    let u = (b.0 - o.0, b.1 - o.1);
    t.0 * u.1 - u.0 * t.1
}

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64),
        d: (i64, i64),
    };
    let prod_min = {
        prod(a, b, d)
            .min(prod(b, c, a))
            .min(prod(c, d, b))
            .min(prod(d, a, c))
    };
    println!("{}", if prod_min > 0 { "Yes" } else { "No" });
}
