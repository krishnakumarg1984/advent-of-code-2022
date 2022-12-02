#[allow(unused_imports)]
use std::{collections::HashMap, error::Error, fs};

// Rock(A,X) Paper(B,Y) Scissors(C,Z)

// Rock(A,X) > Scissors(C,Z)   => A > Z (loss: 0), C < X (victory: 6), A == X (draw: 3)
// Scissors(C,Z) > Paper(B,Y)  => C > Y (loss: 0), B < Z (victory: 6), B == Y (draw: 3)
// Paper(B,Y) > Rock(A,X)      => B > X (loss: 0), A < Y (victory: 6), C == Z (draw: 3)

// A X = 4 [1 (X) + 3 (draw)]
// A Y = 8 [2 (Y) + 6 (victory)]
// A Z = 3 [3 (Z) + 0 (loss)]
// B X = 1 [1 (X) + 0 (loss)]
// B Y = 5 [2 (Y) + 3 (draw)]
// B Z = 9 [3 (Z) + 6 (victory)]
// C X = 7 [1 (Z) + 6 (victory)]
// C Y = 2 [2 (Y) + 0 (loss)]
// C Z = 6 [3 (Z) + 3 (draw)]

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
    points_map.insert("A X", 4);
    points_map.insert("A Y", 8);
    points_map.insert("A Z", 3);

    points_map.insert("B X", 1);
    points_map.insert("B Y", 5);
    points_map.insert("B Z", 9);

    points_map.insert("C X", 7);
    points_map.insert("C Y", 2);
    points_map.insert("C Z", 6);

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
