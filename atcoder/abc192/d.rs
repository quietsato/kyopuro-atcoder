fn main() {
    let (x, m) = {
        let mut x = String::new();
        let mut m = String::new();

        std::io::stdin().read_line(&mut x).unwrap();
        std::io::stdin().read_line(&mut m).unwrap();

        let x: Vec<u64> = x
            .trim_end()
            .chars()
            .map(|c| -> u64 { c.to_digit(10).unwrap().into() })
            .collect();
        let m: u64 = m.trim_end().parse().unwrap();

        (x, m)
    };

    // dbg!(&x);

    let ans = if x.len() == 1 {
        if x[0] <= m as u64 {
            1
        } else {
            0
        }
    } else {
        bin_search(&x, m)
    };

    println!("{}", ans);
}

fn bin_search(x: &Vec<u64>, m: u64) -> usize {
    const MAX: u64 = 2 * 10u64.pow(18);

    let d = x.iter().max().unwrap().clone();

    let mut l = d;
    let mut r = m + 2;

    while r - l > 1 {
        let n = (l + r) / 2;

        let mut t = 0u64;
        for fig in x {
            if t as f64 > MAX as f64 / n as f64 {
                t = MAX;
                break;
            } else {
                t = t * n + fig;
            }
        }
        if t <= m {
            l = n;
        } else {
            r = n;
        }
    }

    (l - d) as usize
}

