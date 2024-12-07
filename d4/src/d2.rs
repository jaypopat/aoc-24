pub fn main() {
    let grid = parse_input();
    let mut count = 0;

    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {

            if grid[row][col] == 'A' && check_xmas(&grid, row, col) {
                count += 1;
            }
        }
    }
    println!("Total X-MAS patterns found: {}", count);
}
fn check_xmas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    // getting the four diagonals around the center
    let ul = grid[row - 1][col - 1];
    let ur = grid[row - 1][col + 1];
    let bl = grid[row + 1][col - 1];
    let br = grid[row + 1][col + 1];

    // checking if diagonals is mas or sam
    let ul_br_valid = (ul == 'M' && br == 'S') || (ul == 'S' && br == 'M');
    let ur_bl_valid = (ur == 'M' && bl == 'S') || (ur == 'S' && bl == 'M');


    ul_br_valid && ur_bl_valid
}

// Parses the input file into a 2D grid of characters
fn parse_input() -> Vec<Vec<char>> {
    let contents = include_str!("../input.txt");

    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
