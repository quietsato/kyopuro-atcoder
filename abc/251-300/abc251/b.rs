use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        mut a: [u64; n],
    }
    let mut good = vec![0; (w + 1) as usize];
    for i in 0..n {
        let x = a[i];
        if x <= w {
            good[x as usize] = 1;
        }
        for j in i + 1..n {
            let x = a[i] + a[j];
            if x <= w {
                good[x as usize] = 1;
            }
            for k in j + 1..n {
                let x = a[i] + a[j] + a[k];
                if x <= w {
                    good[x as usize] = 1;
                }
            }
        }
    }
    println!("{}", good.iter().sum::<u64>());
}
