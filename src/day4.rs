use crate::helpers::read_char_grid;

use std::path::Path;

pub fn day_4_part_1() {
    let file_path: &Path = Path::new("inputs/day4.txt");

    let grid: Vec<Vec<char>> = read_char_grid(file_path).unwrap();

    let mut accessible_count: i32 = 0;

    for row_num in 0..grid.len() {
        for col_num in 0..grid[0].len() {
            if grid[row_num][col_num] != '@' {
                continue;
            }
            let nbr_idxs: Vec<(usize, usize)> = build_nbrs(row_num, col_num, grid.len() - 1, grid[row_num].len() - 1);
            let mut roll_count: i32 = 0;
            for (nbr_x, nbr_y) in nbr_idxs {
                if grid[nbr_x][nbr_y] == '@' {
                    roll_count += 1;
                }
            }
            if roll_count < 4 {
                accessible_count += 1;
            }
        }
    }

    println!("Accessible rolls: {:?}", accessible_count);
}

pub fn day_4_part_2() {
    let file_path: &Path = Path::new("inputs/day4.txt");

    let mut grid: Vec<Vec<char>> = read_char_grid(file_path).unwrap();

    let mut accessible_count: i32 = 0;
    let mut total_accessible: i32 = 0;
    let mut iteration: i32 = 0;

    while iteration < 100 {
        for row_num in 0..grid.len() {
            for col_num in 0..grid[0].len() {
                if grid[row_num][col_num] != '@' {
                    continue;
                }
                let nbr_idxs: Vec<(usize, usize)> = build_nbrs(row_num, col_num, grid.len() - 1, grid[row_num].len() - 1);
                let mut roll_count: i32 = 0;
                for (nbr_x, nbr_y) in nbr_idxs {
                    if grid[nbr_x][nbr_y] != '.' {
                        roll_count += 1;
                    }
                }
                if roll_count < 4 {
                    accessible_count += 1;
                    grid[row_num][col_num] = 'X';
                }
            }
        }
        println!("Iteration {:?}, found {:?} accessible rolls", iteration, accessible_count);
        total_accessible += accessible_count;
        if accessible_count == 0 {
            break;
        }
        // Clean up counters and grid for next iteration
        iteration += 1;
        accessible_count = 0;
        for row_num in 0..grid.len() {
            for col_num in 0..grid[0].len() {
                if grid[row_num][col_num] == 'X' {
                    grid[row_num][col_num] = '.';
                }
            }
        }
    }
    println!("Total accessible rolls: {:?}", total_accessible);
}

fn build_nbrs(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut nbrs: Vec<(usize, usize)> = Vec::new();
    // 3 cells to the left
    if x > 0 {
        if y > 0{
            nbrs.push((x-1, y-1));
        }
        nbrs.push((x-1, y));
        if y < y_max {
            nbrs.push((x-1, y+1));
        }
    }
    // Above and below
    if y > 0 {
        nbrs.push((x, y-1));
    }
    if y < y_max {
        nbrs.push((x, y+1));
    }   
    // 3 cells to the right
    if x < x_max {
        if y > 0{
            nbrs.push((x+1, y-1));
        }
        nbrs.push((x+1, y));
        if y < y_max {
            nbrs.push((x+1, y+1));
        }
    }
    return nbrs;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_nbrs() {
        assert_eq!(build_nbrs(0, 0, 2, 2), vec![(0,1), (1,0), (1,1)]);
        assert_eq!(build_nbrs(0, 1, 2, 2), vec![(0,0), (0,2), (1,0), (1,1), (1,2)]);
        assert_eq!(build_nbrs(1, 0, 2, 2), vec![(0,0), (0,1), (1,1), (2,0), (2,1)]);
        assert_eq!(build_nbrs(1, 1, 2, 2), vec![(0,0), (0,1), (0,2), (1,0), (1,2), (2,0), (2,1), (2,2)]);
        assert_eq!(build_nbrs(2, 1, 2, 2), vec![(1,0), (1,1), (1,2), (2,0), (2,2)]);
        assert_eq!(build_nbrs(1, 2, 2, 2), vec![(0,1), (0,2), (1,1), (2,1), (2,2)]);
        assert_eq!(build_nbrs(2, 2, 2, 2), vec![(1,1), (1,2), (2,1)]);
    }
}