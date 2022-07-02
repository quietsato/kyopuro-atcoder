use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        qx: [(u32, usize); q]
    }

    let mut i = n;
    for (q, x) in qx {
        match q {
            1 => {
                i += n;
                i -= x;
            }
            2 => {
                println!("{}", s[(i + x - 1) % n])
            }
            _ => unreachable!(),
        }
    }
}
