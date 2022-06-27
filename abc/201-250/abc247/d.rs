use proconio::input;

fn main() {
    input! {
        q: usize
    }

    let mut l = 0;
    let mut tube = vec![];

    for _ in 0..q {
        input! {
            q: u8
        }
        match q {
            1 => {
                input! {
                    x: u64,
                    c: u64
                }
                tube.push((x, c));
            }
            2 => {
                input! {
                    mut c: u64
                }

                let mut ans = 0;
                loop {
                    if tube[l].1 >= c {
                        ans += c * tube[l].0;
                        tube[l].1 -= c;
                        break;
                    } else {
                        ans += tube[l].1 * tube[l].0;
                        c -= tube[l].1;
                        l += 1;
                    }
                }

                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
