#[allow(dead_code)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = "".to_string();
    let mut max = usize::MAX;
    for ele in &strs {
        if ele.len() < max {
            max = ele.len();
        }
    }
    for i in 0..max {
        let tmp = strs[0].chars().nth(i).unwrap();
        for j in 1..strs.len() {
            if tmp != strs[j].chars().nth(i).unwrap() {
                return prefix;
            }
        }
        prefix.push(tmp);
    }
    prefix
}

#[cfg(test)]
mod longest_common_prefix_test {
    use super::*;

    #[test]
    fn longest_common_prefix_test() {
        assert_eq!("fl".to_string(), longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]));
        assert_eq!("".to_string(), longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]));
    }
}