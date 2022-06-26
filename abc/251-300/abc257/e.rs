use proconio::input;

fn main() {
    input! {
        mut n: u64,
        c: [u64; 9],
    }

    let min = c.iter().min().unwrap();
    let l = n / min;

    if l == 0 {
        println!("0");
        return;
    }

    let mut ans = vec![];

    for i in 1..=l {
        for j in (1..=9).rev() {
            if (l - i) * min + c[j - 1] <= n {
                n -= c[j - 1];
                ans.push(j.to_string());
            }
        }
    }

    println!("{}", ans.join(""));
}
