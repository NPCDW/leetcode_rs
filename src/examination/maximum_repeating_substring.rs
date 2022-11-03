#[allow(dead_code)]
pub fn max_repeating(sequence: String, word: String) -> i32 {
    let mut max = 0;
    let mut i = 0;
    'out: loop {
        if let Some(index) = sequence[i..].find(&word).map(|u| u + i) {
            let mut count = 1;
            i = index + word.len();
            loop {
                let some = sequence[i..].find(&word);
                if some.is_none() {
                    max = max.max(count);
                    break 'out;
                }
                if some.unwrap() == 0 {
                    count += 1;
                    i += word.len();
                } else {
                    i -= word.len() - 1;
                    max = max.max(count);
                    break;
                }
            }
        } else {
            break;
        }
    }
    max
}

#[cfg(test)]
mod maximum_repeating_substring_test {
    use super::*;

    #[test]
    fn max_repeating_test() {
        // let result = max_repeating("dababc".to_string(), "ab".to_string());
        // assert_eq!(2, result);
        // let result = max_repeating("ababc".to_string(), "ab".to_string());
        // assert_eq!(2, result);
        // let result = max_repeating("dababcababab".to_string(), "ab".to_string());
        // assert_eq!(3, result);
        // let result = max_repeating("dababc".to_string(), "cd".to_string());
        // assert_eq!(0, result);
        // let result = max_repeating("dababc".to_string(), "d".to_string());
        // assert_eq!(1, result);
        let result = max_repeating("aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string(), "aaaba".to_string());
        assert_eq!(5, result);
    }
}