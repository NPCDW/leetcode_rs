#[allow(dead_code)]
pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut res = 1;
    for ele in nums {
        if ele < 0 {
            res *= -1
        } else if ele == 0 {
            res = 0
        }
    }
    res
}