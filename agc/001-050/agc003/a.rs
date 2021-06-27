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
        mut s: proconio::marker::Chars
    };

    let ans: bool = {
        let set: BTreeSet<char> = s.iter().cloned().collect();

        let ns = !(set.contains(&'N') ^ set.contains(&'S'));
        let ew = !(set.contains(&'E') ^ set.contains(&'W'));

        ns && ew
    };

    println!("{}", if ans { "Yes" } else { "No" });
}
