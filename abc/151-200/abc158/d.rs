use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        s: String,
        q: usize
    }

    let (mut head, mut tail) = (VecDeque::new(), VecDeque::new());
    let mut reversed = false;

    for _ in 0..q {
        input! {c: u32}
        match c {
            1 => {
                reversed = !reversed;
            }
            2 => {
                input! {f: usize, c: char}
                if f == 1 {
                    if reversed {
                        tail.push_back(c);
                    } else {
                        head.push_front(c);
                    }
                } else {
                    if reversed {
                        head.push_front(c);
                    } else {
                        tail.push_back(c);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let s = format!(
        "{}{}{}",
        head.iter().collect::<String>(),
        s,
        tail.iter().collect::<String>()
    );
    if reversed {
        println!("{}", s.chars().rev().collect::<String>());
    } else {
        println!("{}", s);
    }
}
