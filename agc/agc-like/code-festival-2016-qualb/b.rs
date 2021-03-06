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
        (_n, a, b): (usize, usize, usize),
        s: proconio::marker::Chars
    };

    let mut passed_participants = 0usize;
    let mut passed_foreigners = 0usize;

    for participant in s {
        let (is_passed, is_foreigner) = match participant {
            'a' => (passed_participants < a + b, false),
            'b' => (passed_participants < a + b && passed_foreigners < b, true),
            _ => (false, false),
        };

        println!("{}", if is_passed { "Yes" } else { "No" });

        if is_passed {
            passed_participants += 1;
            if is_foreigner {
                passed_foreigners += 1;
            }
        }
    }
}

