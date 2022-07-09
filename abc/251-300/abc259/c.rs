use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_spans = vec![(s[0], 1)];
    let mut t_spans = vec![(t[0], 1)];

    for s in s.iter().skip(1) {
        if s_spans.last().unwrap().0 == *s {
            s_spans.last_mut().unwrap().1 += 1;
        } else {
            s_spans.push((*s, 1));
        }
    }
    for t in t.iter().skip(1) {
        if t_spans.last().unwrap().0 == *t {
            t_spans.last_mut().unwrap().1 += 1;
        } else {
            t_spans.push((*t, 1));
        }
    }

    if s_spans.len() != t_spans.len() {
        println!("No");
    } else {
        for (s, t) in s_spans.iter().zip(t_spans.iter()) {
            if s.0 != t.0 {
                println!("No");
                return;
            }
            if s.1 == 1 && t.1 >= 2 {
                println!("No");
                return;
            }
            if s.1 > t.1 {
                println!("No");
                return;
            }
        }
        println!("Yes");
    }
}
