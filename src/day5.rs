use crate::helpers::read_lines;

use std::path::Path;

pub fn day_5_part_1() {
    let file_path: &Path = Path::new("inputs/day5.txt");

    let fresh_ranges: Vec<(i64, i64)> = get_fresh_ranges(file_path);
    let ingredients: Vec<i64> = get_ingredients(file_path);

    let fresh_count: i64 = count_fresh_ingredients(&ingredients, &fresh_ranges);
    
    println!("Fresh ingredients: {:?}", fresh_count);
}

pub fn day_5_part_2() {
    let file_path: &Path = Path::new("inputs/day5.txt");

    let fresh_ranges: Vec<(i64, i64)> = get_fresh_ranges(file_path);

    let fresh_ids: i64 = count_fresh_ids(&fresh_ranges);

    println!("Fresh IDs: {:?}", fresh_ids);
}

fn get_fresh_ranges(file_path: &Path) -> Vec<(i64, i64)> {
    let mut fresh_ranges: Vec<(i64, i64)> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok){
            if line.is_empty(){
                break;
            } else if line.contains("-") {
                let parts: Vec<&str> = line.split('-').collect();
                let start: i64 = parts[0].parse::<i64>().unwrap();
                let end: i64 = parts[1].parse::<i64>().unwrap();
                fresh_ranges.push((start, end));
            }
        }
    }

    return fresh_ranges;
}

fn get_ingredients(file_path: &Path) -> Vec<i64> {
    let mut ingredients: Vec<i64> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok){
            if line.is_empty() || line.contains("-") {
                continue;
            } else {
                let ingredient: i64 = line.parse::<i64>().unwrap();
                ingredients.push(ingredient);
            }
        }
    }

    return ingredients;
}


fn count_fresh_ingredients(ingredients: &Vec<i64>, fresh_ranges: &Vec<(i64, i64)>) -> i64 {
    let mut fresh_count: i64 = 0;
    for ingredient in ingredients {
        for range in fresh_ranges {
            if *ingredient >= range.0 && *ingredient <= range.1 {
                fresh_count += 1;
                break;
            }
        }
    }
    return fresh_count;
}

fn count_fresh_ids(fresh_ranges: &Vec<(i64, i64)>) -> i64 {
    let mut fresh_count: i64 = 0;
    let mut tracker: i64 = -1;

    let mut sorted_ranges = fresh_ranges.clone();
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    for range in sorted_ranges {
        if range.0 > tracker {
            fresh_count += range.1 - range.0 + 1;
            tracker = range.1;
        } else if range.1 >= tracker {
            fresh_count += range.1 - tracker;
            tracker = range.1;
        }
    }

    return fresh_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_fresh_ingredients() {
        
        let fresh_ranges: Vec<(i64, i64)> = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let ingredients: Vec<i64> = vec![1, 5, 8, 11, 17, 32];

        let result: i64 = count_fresh_ingredients(&ingredients, &fresh_ranges);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_fresh_ids() {
        let mut fresh_ranges: Vec<(i64, i64)> = vec![(3, 5)]; 
        let mut result: i64 = count_fresh_ids(&fresh_ranges);
        assert_eq!(result, 3);

        fresh_ranges.push((10, 14));
        result = count_fresh_ids(&fresh_ranges);
        assert_eq!(result, 8);

        fresh_ranges.push((16, 20));
        result = count_fresh_ids(&fresh_ranges);
        assert_eq!(result, 13);

        fresh_ranges.push((12, 18));
        result = count_fresh_ids(&fresh_ranges);
        assert_eq!(result, 14);
    }
}