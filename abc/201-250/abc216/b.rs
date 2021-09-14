use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n]
    }

    let mut flag = false;
    for i in 0..n {
        for j in i+1..n {
            if st[i].0 == st[j].0 && st[i].1 == st[j].1 {
                flag = true;
            }
        }
    }

    println!("{}", if flag { "Yes" } else { "No" });
}
