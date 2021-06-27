#[allow(unused_imports)]
use {proconio::input, std::collections::*};

#[allow(dead_code)]
mod util {}

#[cfg(test)]
mod test {}

#[allow(unused_imports)]
fn main() {
    use util::*;

    input! {
        (k, n): (i64, i64),
        mut a: [i64; n]
    };

    let ans: i64 = {
        a.push(a[0] + k);

        let b = {
            let mut b = a
                .iter()
                .zip(a.iter().skip(1))
                .map(|(x, y)| y - x)
                .collect::<Vec<i64>>();
            b.sort();
            b.pop();
            b
        };

        b.iter().sum()
    };

    println!("{}", ans);
}
