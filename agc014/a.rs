fn main() {
    let (a, b, c) = {
        let mut buf = String::new();

        std::io::stdin().read_line(&mut buf).unwrap();

        let abc: Vec<u64> = buf
            .trim_end()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut iter = abc.iter();

        (
            iter.next().unwrap().to_owned(),
            iter.next().unwrap().to_owned(),
            iter.next().unwrap().to_owned(),
        )
    };

    println!(
        "{}",
        solve(a, b, c).map_or("-1".to_owned(), |ans| ans.to_string())
    );
}

fn solve(a: u64, b: u64, c: u64) -> Option<usize> {
    if a == b && b == c {
        if a.is_even() {
            return None;
        } else {
            return Some(0);
        }
    }

    let ans: usize = {
        let mut i = 0usize;
        let (mut a, mut b, mut c) = (a, b, c);
        while a.is_even() && b.is_even() && c.is_even() {
            let (a_, b_, c_) = (a / 2, b / 2, c / 2);
            a = b_ + c_;
            b = c_ + a_;
            c = a_ + b_;

            i += 1;
        }
        i
    };

    Some(ans)
}

trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u64 {
    fn is_even(&self) -> bool {
        self & 1 == 0
    }
}

