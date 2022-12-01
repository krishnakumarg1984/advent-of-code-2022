use std::{error::Error, fs};

#[allow(clippy::missing_errors_doc)]
pub fn run() -> Result<(), Box<dyn Error>> {
    // let input_filename = "test_input.txt"; // the example given in the problem page
    let input_filename = "input.txt"; // unique input for my authenticated user handle
    let result = calculate_highest_cal(input_filename)?;
    let (idx_highest_cal_elf, highest_totalcals) = result;
    println!("The elf index {idx_highest_cal_elf} has packed the highest calorie food ({highest_totalcals} calories)");

    Ok(())
}

fn calculate_highest_cal(input_filename: &str) -> Result<(usize, u32), Box<dyn Error>> {
    let elf_calories = fs::read_to_string(input_filename)?;

    let calories_strvec: Vec<String> = elf_calories
        .split("\n\n")
        .map(std::string::ToString::to_string)
        .collect();

    let mut highest_totalcals = 0;
    let mut idx_highest_cal_elf = 0;
    for (idx, elem) in calories_strvec.iter().enumerate() {
        let cals_with_elf: u32 = elem.split('\n').flat_map(str::parse::<u32>).sum();
        if idx == 0 {
            highest_totalcals = cals_with_elf;
        } else if cals_with_elf > highest_totalcals {
            highest_totalcals = cals_with_elf;
            idx_highest_cal_elf = idx;
        }
    }
    Ok((idx_highest_cal_elf, highest_totalcals))
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
