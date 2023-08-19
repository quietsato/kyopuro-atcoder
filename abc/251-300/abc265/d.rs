use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        r: u64,
        a: [u64; n],
    };
    let cum = [0]
        .iter()
        .chain(a.iter())
        .scan(0, |s, a| {
            *s += a;
            Some(*s)
        })
        .collect_vec();

    let (mut x, mut y, mut z, mut w) = (0, 1, 2, 3);
    while x <= n - 3 {
        // P
        y = y.max(x + 1);
        if y > n - 2 {
            break;
        }
        match (cum[y] - cum[x]).cmp(&p) {
            std::cmp::Ordering::Less => {
                y += 1;
                continue;
            }
            std::cmp::Ordering::Greater => {
                x += 1;
                continue;
            }
            std::cmp::Ordering::Equal => {}
        }
        // Q
        z = z.max(y + 1);
        if z > n - 1 {
            break;
        }
        match (cum[z] - cum[y]).cmp(&q) {
            std::cmp::Ordering::Less => {
                z += 1;
                continue;
            }
            std::cmp::Ordering::Greater => {
                y += 1;
                continue;
            }
            std::cmp::Ordering::Equal => {}
        }
        // R
        w = w.max(z + 1);
        if w > n {
            break;
        }
        match (cum[w] - cum[z]).cmp(&r) {
            std::cmp::Ordering::Less => {
                w += 1;
                continue;
            }
            std::cmp::Ordering::Greater => {
                z += 1;
                continue;
            }
            std::cmp::Ordering::Equal => {}
        }
        println!("Yes");
        return;
    }
    println!("No");
}
