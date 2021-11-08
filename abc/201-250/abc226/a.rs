use proconio::input;

fn main() {
    input! {
        x: String
    }

    let i = x.split('.').nth(0).unwrap(); 
    let d = x.split('.').nth(1).unwrap(); 

    if d.chars().next() < Some('5') {
        println!("{}", i);
    }else {
        println!("{}", i.parse::<u64>().unwrap() + 1);
    }
}
