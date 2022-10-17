use std::collections::HashSet;

#[allow(dead_code)]
pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut i = 0;
    while i < fruits.len() {
        let mut count = 0;
        let mut types = HashSet::new();
        for j in i..fruits.len() {
            types.insert(fruits.get(j).unwrap());
            if types.len() > 2 {
                break;
            }
            count += 1;
        }
        if count > max {
            max = count;
        }
        i += count - 1;
        let tmp = i.clone();
        if fruits.len() - tmp <= max {
            break;
        }
        while fruits.get(i - 1) == fruits.get(tmp) {
            i -= 1;
            continue;
        }
    }
    max.try_into().unwrap()
}

#[cfg(test)]
mod fruit_into_baskets_test {
    use super::*;

    #[test]
    fn total_fruit_test() {
        assert_eq!(total_fruit(vec![1,2,1]), 3);
        assert_eq!(total_fruit(vec![0,1,2,2]), 3);
    }
}