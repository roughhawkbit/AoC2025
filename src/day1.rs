use crate::helpers::read_lines;

use std::path::Path;

pub fn day_1_part_1() {
    let file_path: &Path = Path::new("inputs/day1.txt");

    let mut pos: i32 = 50;
    let mut pwd: i32 = 0;
    let mut diff: i32;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok){
            diff = line[1..].parse::<i32>().unwrap();
            if line.starts_with("R") {
                pos += diff;
            } else {
                pos -= diff;
            };
            pos = pos % 100;
            if pos == 0 {
                pwd += 1;
            }
        }
    }
    println!("The password is: {:?}", pwd);
}

pub fn day_1_part_2() {
    let file_path: &Path = Path::new("inputs/day1.txt");

    let mut pos: i32 = 50;
    let mut pwd: i32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok){
            pwd += eval_line(&line, &mut pos);
        }
    }
    println!("The password is: {:?}", pwd);
}


fn eval_line(line: &str, pos: &mut i32) -> i32 {
    let diff = line[1..].parse::<i32>().unwrap();
    let mut zero_clicks: i32 = 0;

    if line.starts_with("R") {
        *pos += diff;
        while *pos > 99 {
            *pos -= 100;
            zero_clicks += 1;
        }
    } else {
        let mut on_zero = *pos == 0;
        *pos -= diff;
        while *pos < 0 {
            *pos += 100;
            if on_zero {
                on_zero = false;
            } else {
                zero_clicks += 1;
            }
        }
        if *pos == 0 {
            zero_clicks += 1;
        }
    };

    zero_clicks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_line() {
        let mut pos: i32;

        // Simple moves that don't cross zero
        pos = 50;
        assert_eq!(eval_line("R10", &mut pos), 0);
        assert_eq!(pos, 60);

        pos = 50;
        assert_eq!(eval_line("L10", &mut pos), 0);
        assert_eq!(pos, 40);

        // Simple moves that cross zero
        pos = 90;
        assert_eq!(eval_line("R15", &mut pos), 1);
        assert_eq!(pos, 5);

        pos = 10;
        assert_eq!(eval_line("L15", &mut pos), 1);
        assert_eq!(pos, 95);

        // Simple moves that land on zero
        pos = 93;
        assert_eq!(eval_line("R7", &mut pos), 1);
        assert_eq!(pos, 0);

        pos = 7;
        assert_eq!(eval_line("L7", &mut pos), 1);
        assert_eq!(pos, 0);

        // Large moves that cross zero multiple times
        pos = 50;
        assert_eq!(eval_line("R180", &mut pos), 2);
        assert_eq!(pos, 30);

        pos = 50;
        assert_eq!(eval_line("L180", &mut pos), 2);
        assert_eq!(pos, 70);

        // Large moves that eventually land on zero
        pos = 50;
        assert_eq!(eval_line("R350", &mut pos), 4);
        assert_eq!(pos, 0);

        pos = 23;
        assert_eq!(eval_line("L323", &mut pos), 4);
        assert_eq!(pos, 0);
    }

    #[test]
    fn test_eval_line_part_2_example(){
        let mut pos: i32 = 50;
        // let commands = vec!["L68","L30","R48","L5","R60", "L55", "L1", "L99", "R14", "L82"];
        let mut pwd: i32 = 0;

        pwd += eval_line("L68", &mut pos);
        assert_eq!(pos, 82);
        assert_eq!(pwd, 1);

        pwd += eval_line("L30", &mut pos);
        assert_eq!(pos, 52);
        assert_eq!(pwd, 1);

        pwd += eval_line("R48", &mut pos);
        assert_eq!(pos, 0);
        assert_eq!(pwd, 2);

        pwd += eval_line("L5", &mut pos);
        assert_eq!(pos, 95);
        assert_eq!(pwd, 2);

        pwd += eval_line("R60", &mut pos);
        assert_eq!(pos, 55);
        assert_eq!(pwd, 3);

        pwd += eval_line("L55", &mut pos);
        assert_eq!(pos, 0);
        assert_eq!(pwd, 4);

        pwd += eval_line("L1", &mut pos);
        assert_eq!(pos, 99);
        assert_eq!(pwd, 4);

        pwd += eval_line("L99", &mut pos);
        assert_eq!(pos, 0);
        assert_eq!(pwd, 5);

        pwd += eval_line("R14", &mut pos);
        assert_eq!(pos, 14);
        assert_eq!(pwd, 5);

        pwd += eval_line("L82", &mut pos);
        assert_eq!(pos, 32);
        assert_eq!(pwd, 6);
    }
}