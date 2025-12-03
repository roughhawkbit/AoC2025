use crate::helpers::read_lines;

use std::path::Path;

pub fn day_3_part_1() {
    let file_path: &Path = Path::new("inputs/day3.txt");

    let mut power: i32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok){
            power += eval_line(&line);
        }
    }

    println!("Power: {:?}", power);
}

fn eval_line(line: &str) -> i32 {
    // Find the first battery
    let mut batteries_to_assess: &str = &line[0..line.len()-1];
    let mut pos: usize = find_highest_digit_index(batteries_to_assess);
    let mut result: i32 = 10 * get_nth_digit(batteries_to_assess, pos);

    // Find the second battery
    batteries_to_assess = &line[pos+1..line.len()];
    pos = find_highest_digit_index(batteries_to_assess);
    result += get_nth_digit(batteries_to_assess, pos);
    
    return result;
}

fn find_highest_digit_index(s: &str) -> usize {
    let mut mtch: Option<usize>;
    for i in (1..=9).rev() {
        let chr: char = char::from_digit(i as u32, 10).unwrap();
        mtch = s.chars().position(|c| c == chr);
        if mtch != None {
            return mtch.unwrap();
        }
    }
    return s.len();
}

fn get_nth_digit(s: &str, n: usize) -> i32 {
    let chr: char = s.chars().nth(n).unwrap();
    return chr.to_digit(10).unwrap() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_line() {
        assert_eq!(eval_line("987654321111111"), 98);
        assert_eq!(eval_line("811111111111119"), 89);
        assert_eq!(eval_line("234234234234278"), 78);
        assert_eq!(eval_line("818181911112111"), 92);
    }
}