use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let mut n: usize = parse_line().unwrap();

    let ans: String = {
        let mut ans = vec![];

        while n > 0 {
            n -= 1;
            ans.push((n % 26) as u32);
            n /= 26;
        }

        ans.iter()
            .map(|x| std::char::from_u32(*x + 'a' as u32).unwrap())
            .rev()
            .collect()
    };

    println!("{}", ans);
}
