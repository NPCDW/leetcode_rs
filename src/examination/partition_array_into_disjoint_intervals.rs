#[allow(dead_code)]
pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    'search: loop {
        let mut max = i32::MIN;
        for i in 0..left+1 {
            max = *nums.get(i).unwrap().max(&max);
        }
        for i in (left+1..nums.len()).rev() {
            if nums.get(i).unwrap() < &max {
                left = i;
                continue 'search;
            }
        }
        return (left + 1) as i32;
    }
}

#[cfg(test)]
mod partition_array_into_disjoint_intervals_test {
    use super::*;

    #[test]
    fn partition_disjoint_test() {
        let result = partition_disjoint(vec![5,0,3,8,6]);
        assert_eq!(result, 3);
        let result = partition_disjoint(vec![1,1,1,0,6,12]);
        assert_eq!(result, 4);
    }
}