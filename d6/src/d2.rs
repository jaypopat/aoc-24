use std::collections::{HashMap, HashSet};

pub fn main() {
    let contents = include_str!("../input.txt");
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let origin = get_origin_point(&grid);

    let loop_count = count_loop_causing_positions(&grid, origin);
    println!("Part 2: {:?}", loop_count);
}

fn count_loop_causing_positions(grid: &[Vec<char>], origin: (&str, (usize, usize))) -> usize {
    let mut loop_count = 0;
    let mut new_grid = grid.to_vec(); // making a vec so i can modify it (i cant on the slice)

    let (rows,cols) = (grid.len(),grid[0].len());

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '.' && (i, j) != origin.1 {
                new_grid[i][j] = '#';
                if run_simulation(&new_grid, origin).1 {
                    loop_count += 1;
                }
                new_grid[i][j] = '.';
            }
        }
    }
    loop_count
}


fn run_simulation(grid: &[Vec<char>], origin: (&str, (usize, usize))) -> (usize, bool) {
    let dirs: HashMap<&str, (i32, i32)> =
        [("^", (-1, 0)), ("v", (1, 0)), ("<", (0, -1)), (">", (0, 1))]
            .into_iter()
            .collect();
    let right_map: HashMap<&str, &str> = [("^", ">"), ("v", "<"), ("<", "^"), (">", "v")]
        .into_iter()
        .collect();

    let (mut current_dir, mut pos) = origin;
    let mut visited = HashSet::new();
    let mut state_history = HashSet::new();

    loop {
        visited.insert(pos);
        if !state_history.insert((pos, current_dir)) {
            return (visited.len(), true); // guard came back to same point hence guard looped
        }

        let curr_move = dirs[current_dir];

        let (left_incr, right_incr) = curr_move;
        let (guard_x, guard_y) = pos;

        let new_pos = (guard_x as i32 + left_incr, guard_y as i32 + right_incr);

        if !is_valid_position(grid, new_pos) {
            return (visited.len(), false); // No loop, guard left the grid
        }

        if check_for_obstacle(grid, new_pos) {
            current_dir = right_map[current_dir];
        } else {
            pos = (new_pos.0 as usize, new_pos.1 as usize);
        }
    }
}

fn is_valid_position(grid: &[Vec<char>], pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < grid.len() as i32 && pos.1 >= 0 && pos.1 < grid[0].len() as i32
}

fn check_for_obstacle(grid: &[Vec<char>], new_pos: (i32, i32)) -> bool {
    grid[new_pos.0 as usize][new_pos.1 as usize] == '#'
}

pub fn get_origin_point(grid: &[Vec<char>]) -> (&str, (usize, usize)) {
    let (rows, cols) = (grid.len(), grid[0].len());
    println!("rows: {}, cols: {}", rows, cols);

    for (i, row) in grid.iter().enumerate().take(rows) {
        for (j, &cell) in row.iter().enumerate().take(cols) {
            match cell {
                '<' => return ("<", (i, j)),
                '>' => return (">", (i, j)),
                '^' => return ("^", (i, j)),
                'v' => return ("v", (i, j)),
                _ => continue,
            }
        }
    }

    panic!("No starting point found in the grid")
}
