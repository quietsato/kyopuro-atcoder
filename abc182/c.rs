use itertools::Itertools;
use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: Vec<u32> = parse_line::<String>()?
        .chars()
        .map(|c| c.to_digit(10).unwrap() % 3)
        .collect_vec();

    let x: u32 = n.iter().sum::<u32>() % 3;

    let m = {
        let mut m = vec![0; 3];
        n.iter().for_each(|x| m[*x as usize] += 1);
        m
    };

    let ans = match x {
        0 => Some(0),
        1 => {
            if m[1] > 1 || (m[1] == 1 && (m[2] + m[0] > 0)) {
                Some(1)
            } else if m[2] > 2 || (m[2] == 2 && (m[1] + m[0] > 0)) {
                Some(2)
            } else {
                None
            }
        }
        2 => {
            if m[2] > 1 || (m[2] == 1 && (m[1] + m[0] > 0)) {
                Some(1)
            } else if m[1] > 2 || (m[1] == 2 && (m[2] + m[0] > 0)) {
                Some(2)
            } else {
                None
            }
        }
        _ => unreachable!(),
    };

    println!("{}", ans.unwrap_or(-1));

    Ok(())
}
