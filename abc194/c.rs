#[allow(unused_imports)]
use {itertools, proconio::input, std::collections::*};

#[allow(dead_code)]
mod util {}

#[cfg(test)]
mod test {}

#[allow(unused_imports)]
fn main() {
    use util::*;

    input! {
        n: i64,
        a: [i64; n]
    };

    let ans = {
        let x: i64 = n * a.iter().map(|v| v.pow(2)).sum::<i64>();
        let y: i64 = a.iter().sum::<i64>().pow(2);

        x - y
    };

    println!("{}", ans);
}

