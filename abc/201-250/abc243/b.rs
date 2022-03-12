use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    }

    let mut ans1 = 0;
    for (a, b) in a.iter().zip(b.iter()) {
        if a == b {
            ans1 += 1;
        }
    }
    println!("{}", ans1);

    let mut ans2 = 0;
    for (i, a) in a.iter().enumerate() {
        for (j, b) in b.iter().enumerate() {
            if i == j {
                continue;
            }
            if a == b {
                ans2 += 1;
            }
        }
    }
    println!("{}", ans2);
}
