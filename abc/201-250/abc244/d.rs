use proconio::input;

fn main() {
    input! {
        s1: char,
        s2: char,
        s3: char,
        t1: char,
        t2: char,
        t3: char,
    }

    if s1 == t1 && s2 == t2 && s3 == t3 {
        println!("Yes");
        return;
    }

    if s1 != t1 && s2 != t2 && s3 != t3 {
        println!("Yes");
        return;
    }

    println!("No");
}

