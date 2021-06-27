#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();
    let ab = {
        let mut ab: Vec<(usize, usize)> = vec![];
        for _ in 0..m {
            ab.push(parse_line().unwrap());
        }
        ab
    };

    let k: u32 = parse_line().unwrap();
    let cd = {
        let mut cd: Vec<(usize, usize)> = vec![];
        for _ in 0..k {
            cd.push(parse_line().unwrap());
        }
        cd
    };

    let ans = {
        let mut ans = 0;

        for bit in 0..2u64.pow(k) {
            let mut ans_ = 0;
            let mut plates = vec![0; n];

            for i in 0..k as usize {
                if (bit >> i) & 1 == 0 {
                    plates[cd[i].0 - 1] += 1;
                } else {
                    plates[cd[i].1 - 1] += 1;
                }
            }

            for i in 0..m {
                if plates[ab[i].0 - 1] > 0 && plates[ab[i].1 - 1] > 0 {
                    ans_ += 1;
                }
            }

            ans = ans.max(ans_);
        }

        ans
    };

    println!("{}", ans);
}
