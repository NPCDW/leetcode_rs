use std::collections::HashSet;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let chars = s.chars().collect::<Vec<char>>();
    for start in 0..chars.len() {
        let mut set = HashSet::new();
        for index in start..chars.len() {
            let res = set.insert(chars[index]);
            if !res || index + 1 == chars.len() {
                if max < set.len() {
                    max = set.len()
                }
                break;
            }
        }
    }
    max as i32
}

#[cfg(test)]
mod longest_substring_without_repeating_characters_test {
    use super::*;

    #[test]
    fn length_of_longest_substring_test() {
        let result = length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(3, result);
        let result = length_of_longest_substring("pwwkew".to_string());
        assert_eq!(3, result);
        let result = length_of_longest_substring("bbbbb".to_string());
        assert_eq!(1, result);
        let result = length_of_longest_substring(" ".to_string());
        assert_eq!(1, result);
    }
}