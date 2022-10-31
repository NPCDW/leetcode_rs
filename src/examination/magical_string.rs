pub fn magical_string(n: i32) -> i32 {
    let mut chars = vec!['1', '2', '2'];
    let mut index = 2;
    let mut current = '1';
    let mut count = 1;
    while chars.len() < n as usize {
        if chars[index] == '1' {
            chars.push(current);
            current = if current == '1' { count += 1; '2' } else { '1' };
        } else {
            chars.push(current);
            chars.push(current);
            current = if current == '1' { count += 2; '2' } else { '1' };
        }
        index += 1;
    }
    if chars.len() > n as usize && chars[chars.len() - 1] == '1' {
        count -= 1;
    }
    count
}

#[cfg(test)]
mod magical_string_test {
    use super::*;

    #[test]
    fn magical_string_test() {
        let result = magical_string(6);
        assert_eq!(result, 3);
        let result = magical_string(1);
        assert_eq!(result, 1);
        let result = magical_string(13);
        assert_eq!(result, 6);
    }
}