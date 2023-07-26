use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [String; n],
    };

    let mut vacant = vec![true; d];
    vacant.push(false);

    for s in s {
        for (i, c) in s.chars().enumerate() {
            vacant[i] &= c == 'o';
        }
    }

    let mut ans = 0;
    let mut j = 0;

    for v in vacant {
        if v {
            j += 1;
        } else {
            ans = ans.max(j);
            j = 0;
        }
    }

    println!("{}", ans);
}
