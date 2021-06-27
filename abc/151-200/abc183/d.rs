use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        (n, w): (usize, i64),
        mut stps: [(usize, usize, i64); n]
    }

    let events = {
        let mut events = vec![];

        for (s, t, p) in stps {
            events.push((s, p));
            events.push((t, -p));
        }

        events.sort_by(|a, b| a.0.cmp(&b.0));
        events
    };

    let mut t = 0;
    let mut current_w = 0;

    let mut ev = events.iter();
    loop {
        let event = ev.next();

        if let Some(event) = event {
            if event.0 == t {
                current_w += event.1;
                continue;
            }
        }

        if current_w > w {
            println!("No");
            return Ok(());
        }

        if let Some(event) = event {
            current_w += event.1;
            t = event.0;
        } else {
            break;
        }
    }

    println!("Yes");

    Ok(())
}

