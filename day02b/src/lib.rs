#[allow(unused_imports)]
use std::{collections::HashMap, error::Error, fs};

// Rock(A=1), Paper(B=2), Scissors(C=3)
// Loss(X=0), Draw(Y=3), Win(Z=6)

// Rock(A) > Scissors(C)
// Scissors(C) > Paper(B)
// Paper(B) > Rock(A)

// A X => loss => A C => 3 (C) + 0 (loss) = 3
// A Y => draw => A A => 1 (A) + 3 (draw) = 4
// A Z => win =>  A B => 2 (B) + 6 (win)  = 8

// B X => loss => B A => 1 (A) + 0 (loss) = 1
// B Y => draw => B B => 2 (B) + 3 (draw) = 5
// B Z => win =>  B C => 3 (C) + 6 (win)  = 9

// C X => loss => C B => 2 (B) + 0 (loss) = 2
// C Y => draw => C C => 3 (B) + 3 (draw) = 6
// C Z => win =>  C A => 1 (A) + 6 (win)  = 7

#[allow(clippy::missing_errors_doc)]
pub fn run() -> Result<(), Box<dyn Error>> {
    let input_filename = "input.txt"; // unique input for my authenticated user handle
    println!(
        "Total points obtained for given strategy is: {}",
        compute_game_scores(input_filename)?
    );

    Ok(())
}

#[allow(dead_code)]
fn compute_game_scores(input_filename: &str) -> Result<u32, Box<dyn Error>> {
    let mut points_map = HashMap::new();

    points_map.insert("A X", 3);
    points_map.insert("A Y", 4);
    points_map.insert("A Z", 8);

    points_map.insert("B X", 1);
    points_map.insert("B Y", 5);
    points_map.insert("B Z", 9);

    points_map.insert("C X", 2);
    points_map.insert("C Y", 6);
    points_map.insert("C Z", 7);

    let strategy_filecontents = fs::read_to_string(input_filename)?;

    let strategy_vec: Vec<&str> = strategy_filecontents.trim().split('\n').collect();

    let mut total_points = 0;
    for item in strategy_vec {
        let points_in_round = points_map.get(&item).copied().unwrap_or(0_u32);
        total_points += points_in_round;
        // println!(
        //     "Strategy: {:?}, Points for round: {}",
        //     item, points_in_round
        // );
    }

    Ok(total_points)
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(15, compute_game_scores("test_input.txt").unwrap());
    }
}
