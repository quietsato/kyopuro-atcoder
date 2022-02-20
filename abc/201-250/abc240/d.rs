use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let mut c = 0;
    let mut v: Vec<(u64, u64)> = vec![];
    for a in a {
        if v.is_empty() || v.last().unwrap().0 != a {
            v.push((a, 1));
            c += 1;
        } else {
            let mut l = v.pop().unwrap();
            c -= l.1;
            if l.1 + 1 < a {
                l.1 += 1;
                v.push(l);
                c += l.1;
            }
        }
        println!("{}", c);
    }
}
