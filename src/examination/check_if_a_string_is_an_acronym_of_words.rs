#[allow(dead_code)]
pub fn is_acronym(words: Vec<String>, s: String) -> bool {
    if words.len() != s.len() {
        return false;
    }
    for i in 0..words.len()  {
        if !words[i].starts_with(s.chars().nth(i).unwrap()) {
            return false;
        }
    }
    true
}

#[test]
fn is_acronym_test() {
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
    let result = is_acronym(vec!["alice".to_string(),"bob".to_string(),"charlie".to_string()], "abc".to_string());
    assert_eq!(true, result);
    let result = is_acronym(vec!["never".to_string(),"gonna".to_string(),"give".to_string(),"up".to_string(),"on".to_string(),"you".to_string()], "ngguoy".to_string());
    assert_eq!(true, result);
    let result = is_acronym(vec!["an".to_string(),"apple".to_string()], "a".to_string());
    assert_eq!(false, result);
}