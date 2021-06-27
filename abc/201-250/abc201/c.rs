use proconio::{input, marker::Chars};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        s: Chars
    }

    let ans = (0..10000)
        .filter_map(|mut p| {
            let mut used = vec![false; 10];

            for i in (0..4).rev() {
                let div = 10_i32.pow(i) as usize;
                let idx = p / div;
                if s[idx] == 'x' {
                    return None;
                }
                used[idx] = true;
                p %= div;
            }

            if used
                .iter()
                .zip(s.iter())
                .find(|pair| *pair == (&false, &'o'))
                .is_some()
            {
                None
            } else {
                Some(())
            }
        })
        .count();

    println!("{}", ans);

    Ok(())
}

