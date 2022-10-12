#[allow(dead_code)]
pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let mut list = vec![];
    let chars1 = s1.chars().collect::<Vec<char>>();
    let chars2 = s2.chars().collect::<Vec<char>>();
    let count = s1.chars().count();
    for index in 0..count {
        if chars1.get(index).unwrap() != chars2.get(index).unwrap() {
            list.push(index);
        }
    }
    match list.len() {
        0 => return true,
        2 => return chars1.get(*list.get(0).unwrap()).unwrap() == chars2.get(*list.get(1).unwrap()).unwrap() && chars1.get(*list.get(1).unwrap()).unwrap() == chars2.get(*list.get(0).unwrap()).unwrap(),
        _ => return false,
    }
}

#[cfg(test)]
mod check_if_one_string_swap_can_make_strings_equal_test {
    use super::*;

    #[test]
    fn are_almost_equal_test() {
        let result = are_almost_equal("bank".to_string(), "kanb".to_string());
        assert_eq!(true, result);
        let result = are_almost_equal("attack".to_string(), "defend".to_string());
        assert_eq!(result, false);
        let result = are_almost_equal("kelb".to_string(), "kelb".to_string());
        assert_eq!(result, true);
        let result = are_almost_equal("abcd".to_string(), "dcba".to_string());
        assert_eq!(result, false);
    }
}