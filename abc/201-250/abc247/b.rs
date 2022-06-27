use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n]
    }

    for i in 0..n {
        let mut ans = false;
        for c in [&st[i].0, &st[i].1].iter() {
            let mut flag = true;
            for j in 0..n {
                if i == j {
                    continue;
                }
                if **c == st[j].0 || **c == st[j].1 {
                    flag = false;
                }
            }
            if flag {
                ans = true;
            }
        }
        if !ans {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
