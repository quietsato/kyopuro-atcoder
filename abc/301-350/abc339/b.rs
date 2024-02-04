use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    };
    let mut grid = vec![vec!['.'; w]; h];

    let mut dir = 0usize;
    let mut pos = (0usize, 0usize);

    let diff = [(0i32, -1i32), (1, 0), (0, 1), (-1, 0)];

    for _ in 0..n {
        match grid[pos.1][pos.0] {
            '.' => {
                grid[pos.1][pos.0] = '#';
                dir = ((4 + dir as i32 + 1) % 4) as usize;
            }
            '#' => {
                grid[pos.1][pos.0] = '.';
                dir = ((4 + dir as i32 - 1) % 4) as usize;
            }
            _ => unreachable!(),
        }
        let (dx, dy) = diff[dir];
        pos.0 = ((w + pos.0) as i32 + dx) as usize % w;
        pos.1 = ((h + pos.1) as i32 + dy) as usize % h;
    }

    for g in grid {
        println!("{}", g.iter().collect::<String>());
    }
}
