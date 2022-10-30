#[allow(dead_code)]
pub fn letter_case_permutation(s: String) -> Vec<String> {
    let mut chars = s.chars().collect::<Vec<char>>();
    let mut res = vec![];
    find(&mut chars, &mut res, 0);
    res
}

pub fn find(chars: &mut Vec<char>, res: &mut Vec<String>, index: usize) {
    if index == chars.len() {
        res.push(chars.iter().collect::<String>());
        return;
    }
    if chars[index].is_ascii_digit() {
        find(chars, res, index + 1);
        return;
    }
    chars[index] =  chars[index].to_ascii_lowercase();
    find(chars, res, index + 1);
    chars[index] =  chars[index].to_ascii_uppercase();
    find(chars, res, index + 1);
}

#[cfg(test)]
mod letter_case_permutation_test {
    use super::*;

    #[test]
    fn letter_case_permutation_test() {
        let res = letter_case_permutation("a1b2".to_string());
        println!("{:?}", res);
    }
}