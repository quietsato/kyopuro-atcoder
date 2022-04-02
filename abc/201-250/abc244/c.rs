use std::io::BufReader;

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
    }
    let mut available = vec![true; 2 * n + 1];
    for i in 1..=2 * n + 1 {
        if available[i - 1] {
            println!("{}", i);
            available[i - 1] = false;
        } else {
            continue;
        }
        input! {
            from &mut source,
            x: usize
        }
        if x == 0 {
            return;
        }
        available[x - 1] = false;
    }
}

