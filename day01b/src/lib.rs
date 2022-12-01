use std::{collections::BinaryHeap, error::Error, fs};

#[allow(clippy::missing_errors_doc)]
pub fn run() -> Result<(), Box<dyn Error>> {
    // let input_filename = "test_input.txt"; // the example given in the problem page

    let input_filename = "input.txt"; // unique input for my authenticated user handle

    let n = 3; // use the top 3 elves with the highest calories of food
    println!(
        "Total calories of food with the top {n} elves is {}",
        compute_top_elf_cals(input_filename, n)?
    );

    Ok(())
}

#[allow(dead_code)]
fn compute_top_elf_cals(input_filename: &str, n: u32) -> Result<u32, Box<dyn Error>> {
    let elf_calories = fs::read_to_string(input_filename)?;

    let calories_strvec: Vec<String> = elf_calories
        .split("\n\n")
        .map(std::string::ToString::to_string)
        .collect();

    // Could use a BTreeMap if the indices of the relevant elves are needed
    let mut elf_total_cals = BinaryHeap::<u32>::new();
    for elf_itemised_cals in calories_strvec {
        elf_total_cals.push(
            elf_itemised_cals
                .split('\n')
                .flat_map(str::parse::<u32>)
                .sum(),
        );
    }

    let mut sum_of_top_n_cals = 0;
    // compute the total calories of top 'n' calories
    for _ in 0..n {
        let highest_remaining_total_cal = elf_total_cals.pop().unwrap_or(0);
        sum_of_top_n_cals += highest_remaining_total_cal;
    }

    Ok(sum_of_top_n_cals)
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    //     #[test]
    //     fn one_result() {
    //         let query = "duct";
    //         let contents = "\
    // Rust:
    // safe, fast, productive.
    // Pick three.";
    //
    //         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    //     }
}
