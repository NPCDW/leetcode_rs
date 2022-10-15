#[allow(dead_code)]
pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
    let mut count = 1;
    let mut list = vec![];
    for ele in target {
        list.push("Push".to_string());
        count += 1;
        while ele != count {
            list.push("Pop".to_string());
            count += 1;
            list.push("Push".to_string());
        }
    }
    list
}
