use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _n: usize = parse_line()?;
    let ans: u64 = {
        parse_line::<Vec<u64>>()?
            .iter()
            .scan((0u64, 0u64), |state, &v| {
                if v < state.1 {
                    Some((state.1 - v, state.1.max(v)))
                } else {
                    state.1 = state.1.max(v);
                    Some((0, v))
                }
            })
            .map(|x| x.0)
            .sum()
    };

    println!("{}", ans);

    Ok(())
}
