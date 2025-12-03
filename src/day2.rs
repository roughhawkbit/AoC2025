use crate::helpers::read_comma_list;

use std::path::Path;

pub fn day_2_part_1() {
    let file_path: &Path = Path::new("inputs/day2.txt");
    let mut pwd: i64 = 0;

    if let Ok(items) = read_comma_list(file_path) {
        for item in items.map_while(Result::ok){
            let s = String::from_utf8(item).expect("Our bytes should be valid utf8");
            pwd += eval_item(&s);
        }
    }
    println!("Password: {:?}", pwd);
}

fn eval_item(item: &str) -> i64 {
    let ids: Vec<&str> = item.split('-').collect();
    let mut pwd: i64 = 0;
    for i in ids[0].parse::<i64>().unwrap()..=ids[1].parse::<i64>().unwrap() {
        if is_doubled_pattern(i) {
            pwd += i;
        }
    }
    return pwd;
}

fn is_doubled_pattern(n: i64) -> bool{
    let s: String = n.to_string();
    let nchar = s.len();
    if nchar % 2 != 0 {
        return false;
    }
    return &s[0..(nchar/2)] == &s[(nchar/2)..nchar];
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_doubled_pattern() {
        assert_eq!(is_doubled_pattern(7), false);
        assert_eq!(is_doubled_pattern(11), true);
        assert_eq!(is_doubled_pattern(12), false);
        assert_eq!(is_doubled_pattern(95), false);
        assert_eq!(is_doubled_pattern(99), true);
        assert_eq!(is_doubled_pattern(999), false);
        assert_eq!(is_doubled_pattern(1010), true);
        assert_eq!(is_doubled_pattern(1012), false);
        assert_eq!(is_doubled_pattern(1188511880), false);
        assert_eq!(is_doubled_pattern(1188511885), true);
    }

    #[test]
    fn test_eval_item() {
        assert_eq!(eval_item("11-22"), 11 + 22);
        assert_eq!(eval_item("95-115"), 99);
        assert_eq!(eval_item("998-1012"), 1010);
        assert_eq!(eval_item("1188511880-1188511890"), 1188511885);
        assert_eq!(eval_item("222220-222224"), 222222);
        assert_eq!(eval_item("1698522-1698528"), 0);
        assert_eq!(eval_item("446443-446449"), 446446);
        assert_eq!(eval_item("38593856-38593862"), 38593859);
        assert_eq!(eval_item("565653-565659"), 0);
        assert_eq!(eval_item("824824821-824824827"), 0);
        assert_eq!(eval_item("2121212118-2121212124"), 0);
    }
}