use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let mut p = vec![0; 4];
    let mut ans = 0;
    for a in a {
        p[0] += 1;
        for i in (4 - a)..4 {
            ans += p[i as usize];
            p[i as usize] = 0;
        }
        for i in (0..(4 - a)).rev() {
            p[(i + a) as usize] += p[i as usize];
            p[i as usize] = 0;
        }
    }

    println!("{}", ans);
}
