use whiteread::parse_line;

fn main() {
    let (_, _): (u32, u32) = parse_line().unwrap();
    let a: Vec<usize> = parse_line().unwrap();
    let b: Vec<usize> = parse_line().unwrap();

    let mut map = vec![0; 1000];
    a.iter().for_each(|i| map[i - 1] += 1);
    b.iter().for_each(|i| map[i - 1] += 1);

    let ans: Vec<_> = map
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x == 1 { Some(i + 1) } else { None })
        .collect();
    println!(
        "{}",
        ans.iter()
            .map(|x| format!("{}", x))
            .fold("".to_string(), |mut x, y| {
                x.push_str(&" ");
                x.push_str(&y);
                x
            })
            .trim_start()
    )
}
