use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let n: u32 = parse_line().unwrap();

    let ans = (1..=n)
        .filter_map(|x| {
            if has_7_at_decimal(x) || has_7_at_octal(x) {
                None
            } else {
                Some(())
            }
        })
        .count();

    println!("{}", ans);
}

fn has_7_at_decimal(x: u32) -> bool {
    let mut dec = x;
    while dec > 0 {
        if dec % 10 == 7 {
            return true;
        }
        dec /= 10;
    }
    false
}

fn has_7_at_octal(x: u32) -> bool {
    let mut oct = x;
    while oct > 0 {
        if oct % 8 == 7 {
            return true;
        }
        oct /= 8;
    }

    false
}
