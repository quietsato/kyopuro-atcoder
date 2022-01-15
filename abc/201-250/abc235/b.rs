use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u64; n]
    }

    let mut ans = 0;
    for h in h {
        if h > ans {
            ans = h
        } else {
            break;
        }
    }
    println!("{}", ans);
}
