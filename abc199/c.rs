use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: usize = parse_line()?;
    let s: Vec<char> = parse_line::<String>()?.chars().collect();
    let q: usize = parse_line()?;

    let mut p1 = (0..n).collect_vec();
    let mut p2 = (n..n * 2).collect_vec();
    let mut flip = false;

    for _ in 0..q {
        let (t, a, b) = parse_line::<(usize, usize, usize)>()?;

        match t {
            2 => flip = !flip,
            1 => {
                let (a, b) = (a - 1, b - 1);
                if b < n {
                    if flip {
                        p2.swap(a, b);
                    } else {
                        p1.swap(a, b);
                    }
                } else if a >= n {
                    if flip {
                        p1.swap(a - n, b - n);
                    } else {
                        p2.swap(a - n, b - n);
                    }
                } else {
                    if flip {
                        std::mem::swap(&mut p2[a], &mut p1[b - n]);
                    } else {
                        std::mem::swap(&mut p1[a], &mut p2[b - n]);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    if flip {
        println!(
            "{}{}",
            p2.iter().map(|&i| s[i]).collect::<String>(),
            p1.iter().map(|&i| s[i]).collect::<String>()
        );
    } else {
        println!(
            "{}{}",
            p1.iter().map(|&i| s[i]).collect::<String>(),
            p2.iter().map(|&i| s[i]).collect::<String>()
        );
    }
    Ok(())
}

