use itertools::Itertools;
use std::char::from_digit;
use std::collections::HashMap;
use whiteread::parse_line;

fn main() {
    let (s1, s2, s3) = (
        parse_line::<String>().unwrap(),
        parse_line::<String>().unwrap(),
        parse_line::<String>().unwrap(),
    );

    let mut map = HashMap::new();
    s1.chars().for_each(|c| {
        map.insert(c, 0);
    });
    s2.chars().for_each(|c| {
        map.insert(c, 0);
    });
    s3.chars().for_each(|c| {
        map.insert(c, 0);
    });

    for (i, k) in map.clone().keys().enumerate() {
        map.insert(*k, i);
    }

    if map.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }

    for val in (0..=9).permutations(map.len()) {
        if val[*map.get(&s1.chars().nth(0).unwrap()).unwrap()] == 0
            || val[*map.get(&s2.chars().nth(0).unwrap()).unwrap()] == 0
            || val[*map.get(&s3.chars().nth(0).unwrap()).unwrap()] == 0
        {
            continue;
        }

        let v1 = s1
            .chars()
            .map(|c| from_digit(val[*map.get(&c).unwrap()], 10).unwrap())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let v2 = s2
            .chars()
            .map(|c| from_digit(val[*map.get(&c).unwrap()], 10).unwrap())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let v3 = s3
            .chars()
            .map(|c| from_digit(val[*map.get(&c).unwrap()], 10).unwrap())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        if v1 + v2 == v3 && v1 != 0 && v2 != 0 && v3 != 0 {
            println!("{}\n{}\n{}", v1, v2, v3);
            return;
        }
    }
    println!("UNSOLVABLE");
}
