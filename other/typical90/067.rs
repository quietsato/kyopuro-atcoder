use proconio::input;

// 067 Base 8 to 9 (★2)
fn main() {
    input! {
        mut s: String,
        k: usize
    }

    for _ in 0..k {
        let n = u64::from_str_radix(&s, 8).unwrap();
        s = to_9(n).replace('8', "5");
    }

    println!("{}", s);
}

fn to_9(n: u64) -> String {
    let mut n = n;
    let mut ret = vec![];
    loop {
        ret.push(std::char::from_digit((n % 9) as u32, 10).unwrap());
        n /= 9;
        if n == 0 {
            break;
        }
    }
    ret.iter().rev().collect()
}
