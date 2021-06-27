#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools::*,
    whiteread::*,
    std::*,
    std::collections::*
};

#[allow(dead_code)]
mod util {}

#[cfg(test)]
mod test {}

#[allow(unused_imports)]
fn main() {
    use util::*;

    let (n, _, q): (i64, i64, i64) = parse_line().unwrap();
    let wvs = {
        let mut wvs = vec![];
        for _ in 0..n {
            let wv: (i64, i64) = parse_line().unwrap();
            wvs.push(wv);
        }
        wvs.sort_by(|&(_, v1), &(_, v2)| v2.cmp(&v1));
        wvs
    };
    let xs: Vec<i64> = parse_line().unwrap();

    for _ in 0..q {
        let (l, r): (usize, usize) = parse_line().unwrap();

        let ans = {
            let mut ans = 0i64;

            // available boxes
            let boxes = {
                let mut boxes = xs.to_owned();
                for _ in 0..=(r - l) {
                    boxes.remove(l - 1);
                }
                boxes.sort();
                boxes
            };

            let mut wvs = wvs.to_owned();

            for b in boxes {
                if let Some((i, (_, v))) = wvs.iter().find_position(|&(w, _)| b >= *w) {
                    ans += v;
                    wvs.remove(i);
                }
            }

            ans
        };

        println!("{}", ans);
    }
}
