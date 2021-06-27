use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s: Vec<char> = parse_line::<String>()?.chars().collect();

    let mut ans = 0;
    for i in 0..=8 {
        if s[i] == 'Z' && s[i + 1] == 'O' && s[i + 2] == 'N' && s[i + 3] == 'e' {
            ans += 1
        }
    }

    println!("{}", ans);

    Ok(())
}
