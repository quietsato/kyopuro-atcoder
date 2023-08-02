use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [Chars; n]
    };

    let mut nexts = vec![(1usize, 1usize)];
    let mut visited = vec![vec![false; m]; n];

    while let Some((y, x)) = nexts.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        for &diff in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let (ny, nx) = slide((y as i64, x as i64), diff, &grid, &mut visited);
            if !visited[ny][nx] {
                nexts.push((ny, nx));
            }
        }
    }

    let ans = visited.iter().flatten().filter(|&&b| b).count();
    println!("{}", ans);
}

fn slide(
    (y, x): (i64, i64),
    (dy, dx): (i64, i64),
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    let (ny, nx) = (y + dy, x + dx);
    if !(0..grid.len() as i64).contains(&ny)
        || !(0..grid[0].len() as i64).contains(&nx)
        || grid[ny as usize][nx as usize] == '#'
    {
        (y as usize, x as usize)
    } else {
        visited[y as usize][x as usize] = true;
        slide((ny, nx), (dy, dx), grid, visited)
    }
}
