use proconio::input;

fn main() {
    input! {
        mut p: [u8; 26]
    }
    for p in p.iter_mut() {
        *p -= 1;
        *p += b'a';
    }

    println!("{}", p.iter().map(|p| { *p as char }).collect::<String>())
}
