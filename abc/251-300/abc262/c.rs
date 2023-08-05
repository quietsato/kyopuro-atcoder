use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u64; n]
    };

    let ans1 = {
        let n = a
            .iter()
            .enumerate()
            .filter(|&(i, a)| a - 1 == i as u64)
            .count();
        (n * (n - 1)) / 2
    };

    let ans2 = {
        let mut ans2 = 0;

        for (i, &ai) in a.iter().enumerate() {
            let ai = (ai - 1) as usize;
            let j = ai;
            let aj = (a[j] - 1) as usize;
            if ai != i && i < j && aj == i {
                ans2 += 1;
            }
        }

        ans2
    };

    println!("{}", ans1 + ans2);
}
