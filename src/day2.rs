use crate::helpers::read_comma_list;

use std::path::Path;

pub fn day_2_part_1() {
    let file_path: &Path = Path::new("inputs/day2.txt");
    let mut pwd: i64 = 0;

    if let Ok(items) = read_comma_list(file_path) {
        for item in items.map_while(Result::ok){
            let s = String::from_utf8(item).expect("Our bytes should be valid utf8");
            // pwd += eval_item_part_1(&s);
            pwd += eval_item_part_2(&s);
        }
    }
    println!("Password: {:?}", pwd);
}

fn eval_item_part_1(item: &str) -> i64 {
    let ids: Vec<&str> = item.split('-').collect();
    let mut pwd: i64 = 0;
    for i in ids[0].parse::<i64>().unwrap()..=ids[1].parse::<i64>().unwrap() {
        let s: String = i.to_string();
        let n: &str = s.as_str();
        if n.len() % 2 != 0 {
            continue;
        }
        if is_repeating(n, n.len()/2) {
            pwd += i;
        }
    }
    return pwd;
}

fn eval_item_part_2(item: &str) -> i64 {
    let ids: Vec<&str> = item.split('-').collect();
    let mut pwd: i64 = 0;
    for i in ids[0].parse::<i64>().unwrap()..=ids[1].parse::<i64>().unwrap() {
        let s: String = i.to_string();
        let n: &str = s.as_str();
        for snippet_len in 1..=(n.len()/2) {
            if is_repeating(n, snippet_len) {
                pwd += i;
                break;
            }
        }
    }
    return pwd;
}

fn is_repeating_pattern(n: i64) -> bool{
    let s: String = n.to_string();
    let nchar = s.len();
    for i in 1..=(nchar/2) {
        if is_repeating(&s, i) {
            return true;
        }
    }
    return false;
}

fn is_repeating(n: &str, snippet_len: usize) -> bool{
    let nchar = n.len();
    if nchar % snippet_len != 0 {
        return false;
    }
    let pattern: &str = &n[0..snippet_len];
    let mut snippet: &str;

    for i in 1..(nchar/snippet_len) {
        snippet = &n[(i*snippet_len)..((i+1)*snippet_len)];
        if snippet != pattern {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_repeating() {
        assert_eq!(is_repeating("11", 1), true);
        assert_eq!(is_repeating("22", 1), true);
        assert_eq!(is_repeating("115", 1), false);
        assert_eq!(is_repeating("99", 1), true);
        assert_eq!(is_repeating("1012", 1), false);
        assert_eq!(is_repeating("1012", 2), false);
        assert_eq!(is_repeating("1010", 1), false);
        assert_eq!(is_repeating("1010", 2), true);
        assert_eq!(is_repeating("1212", 1), false);
        assert_eq!(is_repeating("1212", 2), true);
        assert_eq!(is_repeating("123123", 1), false);
        assert_eq!(is_repeating("123123", 2), false);
        assert_eq!(is_repeating("123123", 3), true);
        assert_eq!(is_repeating("123124", 3), false);
    }

    #[test]
    fn test_is_repeating_pattern() {
        assert_eq!(is_repeating_pattern(11), true);
        assert_eq!(is_repeating_pattern(22), true);
        assert_eq!(is_repeating_pattern(115), false);
        assert_eq!(is_repeating_pattern(99), true);
        assert_eq!(is_repeating_pattern(1012), false);
        assert_eq!(is_repeating_pattern(1010), true);
        assert_eq!(is_repeating_pattern(1212), true);
        assert_eq!(is_repeating_pattern(123123), true);
        assert_eq!(is_repeating_pattern(123124), false);
    }

    #[test]
    fn test_eval_item_part_1() {
        assert_eq!(eval_item_part_1("11-22"), 11 + 22);
        assert_eq!(eval_item_part_1("95-115"), 99);
        assert_eq!(eval_item_part_1("998-1012"), 1010);
        assert_eq!(eval_item_part_1("1188511880-1188511890"), 1188511885);
        assert_eq!(eval_item_part_1("222220-222224"), 222222);
        assert_eq!(eval_item_part_1("1698522-1698528"), 0);
        assert_eq!(eval_item_part_1("446443-446449"), 446446);
        assert_eq!(eval_item_part_1("38593856-38593862"), 38593859);
        assert_eq!(eval_item_part_1("565653-565659"), 0);
        assert_eq!(eval_item_part_1("824824821-824824827"), 0);
        assert_eq!(eval_item_part_1("2121212118-2121212124"), 0);
    }

    #[test]
    fn test_eval_item_part_2() {
        assert_eq!(eval_item_part_2("11-22"), 11 + 22);
        assert_eq!(eval_item_part_2("95-115"), 99 + 111);
        assert_eq!(eval_item_part_2("998-1012"), 999 + 1010);
        assert_eq!(eval_item_part_2("1188511880-1188511890"), 1188511885);
        assert_eq!(eval_item_part_2("222220-222224"), 222222);
        assert_eq!(eval_item_part_2("1698522-1698528"), 0);
        assert_eq!(eval_item_part_2("446443-446449"), 446446);
        assert_eq!(eval_item_part_2("38593856-38593862"), 38593859);
        assert_eq!(eval_item_part_2("565653-565659"), 565656);
        assert_eq!(eval_item_part_2("824824821-824824827"), 824824824);
        assert_eq!(eval_item_part_2("2121212118-2121212124"), 2121212121);
    }
}