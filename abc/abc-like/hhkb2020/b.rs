use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (h, w): (usize, usize) = parse_line()?;
    let s = (0..h)
        .map(|_| parse_line().unwrap())
        .map(|s: String| s.chars().collect_vec())
        .collect_vec();

    let ans = {
        let mut ans = 0usize;

        for y in 0..h {
            for x in 0..w {
                if s[y][x] == '.' {
                    if y < h - 1 && s[y + 1][x] == '.' {
                        ans += 1;
                    }
                    if x < w - 1 && s[y][x + 1] == '.' {
                        ans += 1;
                    }
                }
            }
        }

        ans
    };

    println!("{}", ans);

    Ok(())
}
