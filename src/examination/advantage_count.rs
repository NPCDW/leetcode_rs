pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums = nums1.clone();
    nums.sort();
    let mut ids: Vec<usize> = (0..nums2.len()).map(|x| x).collect::<Vec<usize>>();
    ids.sort_by(|i, j| nums2[*i].cmp(&nums2[*j]));
    let mut ans: Vec<i32> = vec![0; nums1.len()];
    let mut left = 0;
    let mut right = nums1.len() - 1;
    for i in 0..nums.len() {
        if nums[i] > nums2[ids[left]] {
            ans[ids[left]] = nums[i];
            left += 1;
        } else {
            ans[ids[right]] = nums[i];
            right -= 1;
        }
    }

    ans
}

#[cfg(test)]
mod advantage_count_test {
    use super::*;

    #[test]
    fn advantage_count_test() {
        let result = advantage_count(vec![2,7,11,15], vec![1,10,4,11]);
        print!("{:?}", result);
        let result = advantage_count(vec![12,24,8,32], vec![13,25,32,11]);
        print!("{:?}", result);
    }
}