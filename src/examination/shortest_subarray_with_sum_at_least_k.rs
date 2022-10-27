#[allow(dead_code)]
pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_len = i32::MAX;
    let mut index = 0;
    while index < nums.len() {
        if nums[index] <= 0 {
            index += 1;
            continue;
        }
        let mut sum = 0;
        for i in index..nums.len() {
            sum += nums[i];
            if sum <= 0 {
                index = i;
                break;
            }
            if sum >= k {
                min_len = min_len.min((i - index + 1) as i32);
                break;
            }
        }
        index += 1;
    }
    if min_len == i32::MAX {
        -1
    } else {
        min_len
    }
}