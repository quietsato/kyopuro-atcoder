use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut a = vec![];

    for _ in 0..q {
        input! {
            c: u8,
        }
        match c {
            1 => {
                input! {
                    x: u32,
                }
                a.push(x);
            }
            2 => {
                input! {
                    k: usize,
                }
                println!("{}", a[a.len() - k]);
            }
            _ => unreachable!(),
        }
    }
}
