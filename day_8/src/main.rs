use std::cmp;
use std::fs;

fn read_input_file() -> String {
    let file_path = "src/input.txt";
    println!("Reading from file {:?}", file_path);
    let contents = fs::read_to_string(file_path).expect("Oh no we didn't find anything");
    return contents;
}

fn iterate_over_row_col(
    trees: &Vec<Vec<u32>>,
    mut seen_trees: Vec<Vec<bool>>,
    is_row: bool,
    start_index: usize,
) -> Vec<Vec<bool>> {
    let mut row = if is_row { start_index } else { 0 };
    let mut col = if is_row { 0 } else { start_index };
    let delta = [is_row as usize, !is_row as usize];
    let mut tallest = trees[col][row];
    seen_trees[col][row] = true;
    (row, col) = (delta[1] + row, delta[0] + col);
    while col < trees.len() && row < trees.len() {
        if trees[col][row] > tallest {
            tallest = trees[col][row];
            seen_trees[col][row] = true;
        }
        (row, col) = (delta[1] + row, delta[0] + col);
    }
    // same thing but backwards
    let mut row = if is_row { start_index } else { trees.len() - 1 };
    let mut col = if is_row { trees.len() - 1 } else { start_index };
    let delta = [-(is_row as i32), -(!is_row as i32)];
    tallest = trees[col][row];
    seen_trees[col][row] = true;
    (row, col) = (
        ((delta[1] + row as i32) as usize),
        ((delta[0] + col as i32) as usize),
    );
    while col > 0 && row > 0 {
        if trees[col][row] > tallest {
            tallest = trees[col][row];
            seen_trees[col][row] = true;
        }
        (row, col) = (
            ((delta[1] + row as i32) as usize),
            ((delta[0] + col as i32) as usize),
        );
    }
    return seen_trees;
}

fn problem_1(input: &str) -> u32 {
    // parse input into 2d array
    let grid: Vec<Vec<u32>> = input
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|row| row.chars().map(|num| num.to_digit(10).unwrap()).collect())
        .collect();
    let mut seen_trees = vec![vec![false; grid.len()]; grid.len()];
    for i in 0..grid.len() {
        seen_trees = iterate_over_row_col(&grid, seen_trees.to_owned(), true, i);
        seen_trees = iterate_over_row_col(&grid, seen_trees.to_owned(), false, i);
    }
    return seen_trees
        .iter()
        .map(|row| row.iter().map(|x| *x as u32).sum::<u32>())
        .sum();
}

fn in_bounds(index: i32, grid_size: usize) -> bool {
    return index >= 0 && index < grid_size as i32;
}

fn seen_trees_from_position(col: usize, row: usize, grid: Vec<Vec<u32>>) -> u32 {
    let position_height = grid[row][col];
    let grid_size = grid.len();
    let mut fitness_score = 1;

    // in 4 directions count ho
    for (delta_row, delta_col) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut seen_trees = 0;
        let mut current_col: i32 = col as i32 + delta_col;
        let mut current_row: i32 = row as i32 + delta_row;
        while in_bounds(current_col, grid_size)
            && in_bounds(current_row, grid_size)
            && grid[current_row as usize][current_col as usize] < position_height
        {
            seen_trees += 1;
            current_col = current_col + delta_col;
            current_row = current_row + delta_row;
        }
        if in_bounds(current_col, grid_size) && in_bounds(current_row, grid_size) {
            // one more tree since the last one was just bigger
            seen_trees += 1
        }

        fitness_score *= seen_trees;
    }

    return fitness_score;
}

fn problem_2(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|row| row.chars().map(|num| num.to_digit(10).unwrap()).collect())
        .collect();
    // we just want a 2d max
    let mut max: u32 = 0;

    for row in 0..grid.len() {
        for col in 0..grid.len() {
            let score_at_position =seen_trees_from_position(col, row, grid.clone());
            max = cmp::max(max, score_at_position);
        }
    }
    return max;
}

fn main() {
    let input = read_input_file();
    let solution_1 = problem_1(&input);
    println!("Problem1: {:?}", solution_1);
    let solution_2 = problem_2(&input);
    println!("Problem2: {:?}", solution_2);
}
