use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s: String = parse_line()?;
    let sd = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let map = {
        let mut map = vec![0; 10];

        for d in &sd {
            map[*d] += 1;
        }

        map
    };

    let mut ans = false;

    if s.len() < 3 {
        // |S| < 3
        if s.parse::<u32>()? % 8 == 0
            || s.chars().rev().collect::<String>().parse::<u32>()? % 8 == 0
        {
            ans = true;
        }
    } else {
        // |S| >= 3
        for i in (100 / 8)..(1000 / 8) {
            let t = (i * 8)
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec();

            let mut m = vec![0; 10];
            for x in &t {
                m[*x] += 1;
            }

            ans = true;
            for (a, b) in m.iter().zip(map.iter()) {
                if a > b {
                    ans = false;
                }
            }

            if ans {
                break;
            }
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }

    Ok(())
}
