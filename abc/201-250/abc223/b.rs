use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut ss = vec![];
    for i in 0..s.len() {
        ss.push(format!(
            "{}{}",
            s.chars().skip(i).collect::<String>(),
            s.chars().take(i).collect::<String>()
        ));
    }

    println!("{}", &ss.iter().min().unwrap());
    println!("{}", &ss.iter().max().unwrap());
}
