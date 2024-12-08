pub fn main() {
    let grid = parse_input();

    let antennas = find_antennas(&grid);

    println!("{:?}", antennas);

    let antinodes = find_antinodes(&antennas);

    // println!("{:?}",antinodes);
    //println!("{}",antinodes.len());
}

fn find_antennas(grid: &[Vec<char>]) -> Vec<(char, (u32, u32))> {
    let mut antennas = vec![];

    let (rows, cols) = (grid.len(), grid[0].len());

    for (i, row) in grid.iter().enumerate().take(rows) {
        for (j, &cell) in row.iter().enumerate().take(cols) {
            if cell != '.' {
                antennas.push((cell, (i as u32, j as u32)));
            }
        }
    }

    antennas
}
pub fn find_antinodes(antennas: &[(char, (u32, u32))]) -> Vec<(char, (u32, u32))> {
    let antinodes = vec![];

    todo!("manipulate the antennas to find their corrersponding antinodes");

    antinodes
}

fn parse_input() -> Vec<Vec<char>> {
    let contents = include_str!("../input.txt");

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    grid
}
