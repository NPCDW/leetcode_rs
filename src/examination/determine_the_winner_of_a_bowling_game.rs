#[allow(dead_code)]
pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
    let score1 = compute_score(player1);
    let score2 = compute_score(player2);
    if score1 > score2 {
        1
    } else if score1 < score2 {
        2
    } else {
        0
    }
}

fn compute_score(player: Vec<i32>) -> i32 {
    let mut ten = 0;
    let mut sum = 0;
    for score in player {
        if ten > 0 {
            sum += score * 2;
        } else {
            sum += score;
        }
        if score == 10 {
            ten = 2;
        } else {
            ten -= 1;
        }
    }
    sum
}

#[cfg(test)]
mod determine_the_winner_of_a_bowling_game_test {
    use super::*;

    #[test]
    fn is_winner_test() {
        assert_eq!(is_winner(vec![4,10,7,9], vec![6,5,2,3]), 1);
        assert_eq!(is_winner(vec![3,5,7,6], vec![8,10,10,2]), 2);
        assert_eq!(is_winner(vec![2,3], vec![4,1]), 0);
    }
    
}