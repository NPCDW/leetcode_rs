#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut cur = 0;
    let mut num = x.clone();
    while num != 0 {
        cur = cur * 10 + num % 10;
        num /= 10;
    }
    cur == x
}

#[cfg(test)]
mod palindrome_number_test {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        assert_eq!(is_palindrome(121), true);
    }
}