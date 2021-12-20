use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        p: i64,
        q: i64,
        r: i64,
        s: i64,
    }

    let (l1, r1) = ((1 - a).max(1 - b), (n - a).min(n - b));
    let (l2, r2) = ((1 - a).max(b - n), (n - a).min(b - 1));

    for i in p..=q {
        for j in r..=s {
            let ka = i - a;
            let kb = j - b;
            if ka.abs() != kb.abs() {
                print!(".");
                continue;
            }

            if (l1 <= ka && ka <= r1) || (l2 <= ka && ka <= r2) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

