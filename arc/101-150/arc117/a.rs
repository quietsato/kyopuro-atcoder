use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (a, b): (i32, i32) = parse_line().unwrap();

    let ans = {
        let mut ans = vec![];

        for i in 1..a.min(b) {
            ans.push(i);
            ans.push(-i);
        }

        let (max, min) = (a.max(b), a.min(b));
        let mut min_elem = (min..=max).sum();
        let mut max_elems: Vec<i32> = (min..=max).collect();

        if a < b {
            max_elems.iter_mut().for_each(|x| *x *= -1);
        } else {
            min_elem *= -1;
        }

        ans.push(min_elem);
        ans.append(&mut max_elems);
        ans
    };

    let mut s = String::from("");
    ans.iter().for_each(|x| {
        s.push_str(&x.to_string());
        s.push(' ')
    });
    println!("{}", s.trim_end());
}
