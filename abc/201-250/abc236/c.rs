use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m]
    }

    let mut ti = 0;

    for si in 0..n {
        if s[si] == t[ti] {
            println!("Yes");
            ti += 1;
        } else {
            println!("No");
        }
    }
}
