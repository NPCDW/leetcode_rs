/// 可以将改题目看成一个完全二叉树，每层都有 2 的 n-1 次方个节点，
/// 每个节点的父节点的位置为 (k + 1) / 2 
/// 我们可以根据给定的第几层，第几个不断的寻找他的父节点，直到根节点位置，
/// 然后在从父节点找回叶子节点，并给每一个节点赋值
#[allow(dead_code)]
pub fn kth_grammar(n: i32, k: i32) -> i32 {
    let mut nodes = vec![];
    let mut k = k;
    for _ in 1..n {
        nodes.push(if k % 2 == 0 { 'r' } else { 'l' });
        k = (k + 1) / 2;
    }
    let mut result = 0;
    for ele in nodes.iter().rev() {
        result = if ele == &'l' && result == 0 {
            0
        } else if ele == &'r' && result == 1 {
            0
        } else {
            1
        }
    }
    result
}

#[cfg(test)]
mod k_th_symbol_in_grammar_test {
    use super::*;

    #[test]
    fn kth_grammar_test() {
        assert_eq!(kth_grammar(3, 2), 1);
        assert_eq!(kth_grammar(4, 4), 0);
    }
}

// 0
// 0 1
// 0 1 1 0
// 0 1 1 0 1 0 0 1