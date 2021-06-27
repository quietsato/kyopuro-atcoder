#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools::*,
    whiteread::*,
    std::collections::*
};

#[allow(dead_code)]
mod util {}

#[cfg(test)]
mod test {}

#[allow(unused_imports)]
fn main() {
    use util::*;

    let (k, s, t) = (
        parse_line::<i64>().unwrap(),
        parse_line::<String>().unwrap(),
        parse_line::<String>().unwrap(),
    );

    let cards = gen_cards(k, &s, &t);

    let ans = solve(k, &cards, &s[0..4], &t[0..4]);

    println!("{}", ans);
}

fn solve(k: i64, cards: &[i64], s: &str, t: &str) -> f64 {
    let (s, t) = (
        s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec(),
        t.chars().map(|c| c.to_digit(10).unwrap()).collect_vec(),
    );

    let (mut den, num) = (0f64, ((9 * k - 8) * (9 * k - 9)) as f64);

    for s_next in 0u32..9 {
        for t_next in 0u32..9 {
            let (s_point, t_point) = (calc_point(&s, s_next + 1), calc_point(&t, t_next + 1));

            if s_point > t_point {
                if s_next == t_next {
                    den += ((cards[s_next as usize]) * (cards[s_next as usize] - 1)) as f64;
                } else {
                    den += ((cards[s_next as usize]) * (cards[t_next as usize])) as f64;
                }
            }
        }
    }

    den / num
}

fn calc_point(cs: &[u32], c: u32) -> i64 {
    let mut point = 0;
    let mut cs = cs.to_owned();
    for d in 1..=9 {
        cs.push(c);

        let d_count = cs.iter().filter(|&&c| c == d).count() as u32;
        point += d as i64 * 10_i64.pow(d_count);

        cs.pop();
    }
    point
}

fn gen_cards(k: i64, s: &str, t: &str) -> Vec<i64> {
    let mut cards = vec![k; 9];

    for c in s.chars().take(4).map(|c| c.to_digit(10).unwrap()) {
        cards[(c - 1) as usize] -= 1i64;
    }

    for c in t.chars().take(4).map(|c| c.to_digit(10).unwrap()) {
        cards[(c - 1) as usize] -= 1i64;
    }

    cards
}

