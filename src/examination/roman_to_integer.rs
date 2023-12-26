use std::collections::HashMap;

#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let mut map = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);
    let chars = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut sum = 0;
    loop {
        if i == chars.len() - 1 {
            sum += map[&chars[i]];
            break;
        }
        if map[&chars[i]] < map[&chars[i + 1]] {
            sum -= map[&chars[i]];
        } else {
            sum += map[&chars[i]];
        }
        i += 1;
    }
    sum
}

#[allow(dead_code)]
pub fn roman_to_int2(s: String) -> i32 {
    let mut res = 0;
    let char_to_int = |c| -> i32 {
        match c {
            'I' =>  1, 'V' =>  5, 'X' =>  10, 'L' =>  50,
            'C' => 100, 'D' => 500, 'M' => 1000, _ => 0
        }
    };
    let numbers: Vec<i32> = s.chars().map(|c| char_to_int(c)).collect();
    for i in 0..numbers.len() {
        // println!("{} {}", i, res);
        if i + 1 < numbers.len() && numbers[i+1] > numbers[i] {
            res -= numbers[i];
        } else{
            res += numbers[i];
        }
    }
    return res;
}

#[cfg(test)]
mod roman_to_integer_test {
    use super::*;

    #[test]
    fn roman_to_integer_test() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("IX".to_string()), 9);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
    
}