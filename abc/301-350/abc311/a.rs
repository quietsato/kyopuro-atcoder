use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    };

    let mut flag = vec![0; 3];

    for (i, c) in s.chars().enumerate() {
        flag[(c as u8 - b'A') as usize] += 1;
        if flag.iter().all(|&c| c > 0) {
            println!("{}", i + 1);
            break;
        }
    }
}
