use itertools::Itertools;
use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n]
    }

    let map_c = {
        let mut map_c = vec![0; n];
        for c in c {
            map_c[c as usize - 1] += 1;
        }
        map_c
    };

    let map_b = {
        let mut map_b = vec![0; n];
        for (i, b) in b.iter().enumerate() {
            map_b[*b as usize - 1] += map_c[i as usize];
        }
        map_b
    };

    // dbg!(&map_c, &map_b);

    let map_a = {
        let mut map_a = vec![0; n];
        for a in a {
            map_a[a as usize - 1] += 1;
        }
        map_a
    };

    println!(
        "{}",
        map_a
            .iter()
            .zip(map_b.iter())
            .map(|(&a, &b)| { a * b })
            .sum::<i64>()
    );

    Ok(())
}

