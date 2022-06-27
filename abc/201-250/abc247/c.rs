use proconio::input;

fn main() {
    input! {
        n: u32
    }
    println!("{}", f(n));
}

fn f(n: u32) -> String {
    match n {
        1 => {
            "1".into()
        },
        n => {
            format!("{} {} {}", f(n - 1), n, f(n - 1))
        }
    }
} 
