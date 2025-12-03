use crate::helpers::read_lines;

use std::path::Path;

pub fn day_3_part_1() {
    let file_path: &Path = Path::new("inputs/day3.txt");

    let mut power: i64 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok){
            power += eval_line(&line, 12);
        }
    }

    println!("Power: {:?}", power);
}

fn eval_line(line: &str, nbatteries: i32) -> i64 {
    
    
    let mut abs_pos: usize = 0;
    let mut rel_pos: usize;
    
    let mut batteries_to_assess: &str;
    let mut exponent: u32;

    let mut result: i64 = 0;
    

    for i in (1..=nbatteries).rev() {
        exponent = (i-1).try_into().unwrap();
        batteries_to_assess = &line[abs_pos..line.len()-(i-1) as usize];
        rel_pos = find_highest_digit_index(batteries_to_assess);
        abs_pos += rel_pos;
        result += get_nth_digit(line, abs_pos) * 10_i64.pow(exponent);
        abs_pos += 1;
    }
    
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

fn get_nth_digit(s: &str, n: usize) -> i64 {
    let chr: char = s.chars().nth(n).unwrap();
    return chr.to_digit(10).unwrap() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_line() {
        assert_eq!(eval_line("987654321111111", 2), 98);
        assert_eq!(eval_line("811111111111119", 2), 89);
        assert_eq!(eval_line("234234234234278", 2), 78);
        assert_eq!(eval_line("818181911112111", 2), 92);

        assert_eq!(eval_line("987654321111111", 12), 987654321111);
        assert_eq!(eval_line("811111111111119", 12), 811111111119);
        assert_eq!(eval_line("234234234234278", 12), 434234234278);
        assert_eq!(eval_line("818181911112111", 12), 888911112111);
    }
}