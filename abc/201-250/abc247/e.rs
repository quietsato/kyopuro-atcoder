use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
        mut a: [u64; n]
    }

    let sub = {
        let mut sp = a
            .iter()
            .enumerate()
            .filter_map(|(i, a)| if (y..=x).contains(a) { None } else { Some(i) })
            .collect_vec();
        sp.push(n);

        let mut sub = vec![];
        let mut l = 0;
        for &r in &sp {
            if l == r {
                l += 1;
            } else {
                sub.push((l, r));
                l = r + 1;
            }
        }
        sub
    };

    let mut ans = 0;

    for (ll, rr) in sub {
        let mut count_x = 0;
        let mut count_y = 0;

        let mut r = ll;
        for l in ll..rr {
            while r < rr && (count_x == 0 || count_y == 0) {
                if a[r] == x {
                    count_x += 1;
                }
                if a[r] == y {
                    count_y += 1;
                }
                r += 1;
            }
            if count_x > 0 && count_y > 0 {
                ans += rr - (r - 1);
            }

            if a[l] == x {
                count_x -= 1;
            }
            if a[l] == y {
                count_y -= 1;
            }
        }
    }

    println!("{}", ans);
}
