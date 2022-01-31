use proconio::input;

fn main() {
    input! {
        s: String
    }

    let s1 = s.trim_matches(|c| c == 'a').to_owned();
    let t1 = s1.chars().rev().collect::<String>();

    if s1 == t1 {
        let s2 = s.chars().take_while(|&c| c == 'a').count();
        let t2 = s.chars().rev().take_while(|&c| c == 'a').count();

        if s2 <= t2 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
