#[allow(dead_code)]
pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::default();
    let word1 = word1.chars().collect::<Vec<char>>();
    let word2 = word2.chars().collect::<Vec<char>>();
    let len = if word1.len() > word2.len() { word1.len() } else { word2.len() };
    for index in 0..len {
        if word1.len() > index {
            result.push(*word1.get(index).unwrap());
        }
        if word2.len() > index {
            result.push(*word2.get(index).unwrap());
        }
    }
    result
}