use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(usize, Usize1); q],
    }

    for (t, k) in tk {
        let mut lk = k;
        let mut i = vec![lk];
        loop {
            if lk == 0 {
                break;
            }
            lk /= 2;
            i.push(lk);
        }
        i = i.into_iter().take(t + 1).collect_vec();
        i.reverse();

        if t == 0 {
            println!("{}", s[k]);
            continue;
        }

        let mut ans = if i.len() > t {
            s[i[0]]
        } else {
            match s[i[0]] {
                'A' => match (t - i.len() + 1) % 3 {
                    0 => 'A',
                    1 => 'B',
                    2 => 'C',
                    _ => unreachable!(),
                },
                'B' => match (t - i.len() + 1) % 3 {
                    0 => 'B',
                    1 => 'C',
                    2 => 'A',
                    _ => unreachable!(),
                },
                'C' => match (t - i.len() + 1) % 3 {
                    0 => 'C',
                    1 => 'A',
                    2 => 'B',
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        };

        for i in i.iter().skip(1) {
            ans = match ans {
                'A' => {
                    if i & 1 == 0 {
                        'B'
                    } else {
                        'C'
                    }
                }
                'B' => {
                    if i & 1 == 0 {
                        'C'
                    } else {
                        'A'
                    }
                }
                'C' => {
                    if i & 1 == 0 {
                        'A'
                    } else {
                        'B'
                    }
                }
                _ => unreachable!(),
            }
        }

        println!("{}", ans);
    }
}

