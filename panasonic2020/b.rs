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
        (h, w): (u64, u64)
    };

    let ans: u64 = if h == 1 || w == 1 {
        1
    } else {
        let odd: u64 = (w + 1) / 2; // line 1, 3, 5,...
        let even: u64 = w / 2; // line 2, 4, 6, ...

        let x = (odd + even) * (h / 2);

        if h % 2 == 0 {
            x
        } else {
            x + odd
        }
    };

    println!("{}", ans);
}
