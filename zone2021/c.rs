use whiteread::parse_line;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s: String = parse_line()?;

    let mut t = std::collections::VecDeque::new();
    let mut flip = false;

    for c in s.chars() {
        match c {
            'R' => {
                flip = !flip;
            }
            _ if t.is_empty() => t.push_back(c),
            _ if flip => {
                if t.front().unwrap() == &c {
                    t.pop_front().unwrap();
                } else {
                    t.push_front(c);
                }
            }
            _ => {
                if t.back().unwrap() == &c {
                    t.pop_back();
                } else {
                    t.push_back(c);
                }
            }
        }
    }

    let t: String = if flip {
        t.into_iter().rev().collect()
    } else {
        t.into_iter().collect()
    };

    println!("{}", t);

    Ok(())
}
