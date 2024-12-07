pub fn main() {
    let grid = parse_input(); // 2d grid of characters
    let target = "XMAS";

    let dirs = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            for &(left, right) in &dirs {
                // destructured tuple
                if check_word(&grid, row, col, left, right, target) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
fn check_word(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    x_dir: i32,
    y_dir: i32,
    target_str: &str,
) -> bool {
    for (i, ch) in target_str.chars().enumerate() {
        // x:0,M:1,A:2,S:3
        let (new_row, new_col) = (
            (row as i32 + i as i32 * y_dir) as usize,
            (col as i32 + i as i32 * x_dir) as usize,
        );
        let (rows, cols) = (grid.len(), grid[0].len());
        if new_row >= rows || new_col >= cols || grid[new_row][new_col] != ch {
            return false;
        }
    }
    true
}

fn parse_input() -> Vec<Vec<char>> {
    let contents = include_str!("../input.txt");

    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
