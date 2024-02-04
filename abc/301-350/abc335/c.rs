use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut pos = vec![];
    for i in (1..=n).rev() {
        pos.push((i as i64, 0i64));
    }

    for _ in 0..q {
        input! {
            q: u8,
        }
        match q {
            1 => {
                input! {
                    c: char
                }
                let last = pos.last().unwrap();
                match c {
                    'R' => pos.push((last.0 + 1, last.1)),
                    'L' => pos.push((last.0 - 1, last.1)),
                    'U' => pos.push((last.0, last.1 + 1)),
                    'D' => pos.push((last.0, last.1 - 1)),
                    _ => unreachable!(),
                }
            }
            2 => {
                input! {
                    p: usize
                }
                let p = pos[pos.len() - p];
                println!("{} {}", p.0, p.1);
            }
            _ => unreachable!(),
        }
    }
}
