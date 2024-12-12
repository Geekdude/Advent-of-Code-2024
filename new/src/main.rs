use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filename to read the input from.
    filename: Option<String>,
}

fn main() {

    let cli = Args::parse();

    let filename = cli.filename.unwrap_or("files/test_input.txt".to_string());

    part_1(&filename);

    part_2(&filename);

}

fn part_1 (filename: &str) -> i32 {

    let _contents = read_from_file(filename);

    // let value = calcualte_value_part_1(&contents);

    // println!("Part 1 Solution: {value}");
    // value

    1
}

fn part_2 (filename: &str) -> i32 {

    let _contents = read_from_file(filename);

    // let value = calcualte_value_part_2(&contents);

    // println!("Part 2 Solution: {value}");
    // value

    2
}

fn read_from_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Unable to read file.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_test_input() {
        let answer = part_1("files/test_input.txt");
        assert!(answer == 161);
    }

    #[test]
    fn test_part_1_real_input() {
        // let answer = part_1("files/day3_input.txt");
        // assert!(answer == 181345830);
    }

    #[test]
    fn test_part_2_test_input() {
        // let answer = part_2("files/test2_input.txt");
        // assert!(answer == 48);
    }

    #[test]
    fn test_part_2_real_input() {
        // let _answer = part_2("files/day3_input.txt");
        // // assert!(answer == 181345830);
    }
}

