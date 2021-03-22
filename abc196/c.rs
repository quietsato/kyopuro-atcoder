#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools::*,
    whiteread::*,
    std::*,
};

#[allow(dead_code)]
mod util {}

#[cfg(test)]
mod test {}

#[allow(unused_imports)]
fn main() {
    use util::*;

    let n: String = parse_line().unwrap();
    let nl = n.len();

    let ans = {
        let head: u64 = if nl % 2 == 0 {
            n[..(nl / 2)].parse().unwrap()
        } else {
            vec!['9'; nl / 2]
                .into_iter()
                .collect::<String>()
                .parse()
                .unwrap_or(0)
        };
        let tail: u64 = if nl % 2 == 0 {
            n[(nl / 2)..].parse().unwrap()
        } else {
            head
        };

        if head > tail {
            head - 1
        } else {
            head
        }
    };

    println!("{}", ans);
}
