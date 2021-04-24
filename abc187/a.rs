use whiteread::parse_line;

fn main() {
    let (a, b): (i32, i32) = parse_line().unwrap();

    let (a, b) = (a.to_string(), b.to_string());

    println!(
        "{}",
        u32::max(
            a.chars()
                .map(|c| -> u32 { c.to_digit(10).unwrap().into() })
                .sum(),
            b.chars()
                .map(|c| -> u32 { c.to_digit(10).unwrap().into() })
                .sum()
        )
    );
}
