use clap::Parser;
use regex::Regex;
use itertools::Itertools;
use std::collections::BTreeMap;
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

    let contents = read_from_file(filename);

    let value = calculate_value_part_1(&contents);

    println!("Part 1 Solution: {value}");
    value
}

fn part_2 (filename: &str) -> i32 {

    let contents = read_from_file(filename);

    let value = calculate_value_part_2(&contents);

    println!("Part 2 Solution: {value}");
    value
}

fn read_from_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Unable to read file.")
}

fn calculate_value_part_1(contents: &str) -> i32 {
    let re = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();

    re.captures_iter(contents).map(|m| {
        let (_,[v1, v2]) = m.extract(); let v1:i32 = v1.parse().unwrap(); let v2:i32 = v2.parse().unwrap(); v1*v2
    }
    ).sum()
}

enum Control {
    Do,
    Dont,
}

fn calculate_value_part_2(contents: &str) -> i32 {
    let re = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut control = BTreeMap::new();
    control.insert(0, Control::Do);
    control.insert(contents.len(), Control::Do);

    for i in re_do.find_iter(contents) {
        control.insert(i.start(), Control::Do);
    }

    for i in re_dont.find_iter(contents) {
        control.insert(i.start(), Control::Dont);
    }

    let mut sum = 0;

    for ((i1, t1),(i2, _t2)) in control.iter().tuple_windows() {
        if let Control::Do = t1 {
            sum += re.captures_iter(contents.get(*i1..*i2).unwrap()).map(|m| {
                let (_,[v1, v2]) = m.extract(); let v1:i32 = v1.parse().unwrap(); let v2:i32 = v2.parse().unwrap(); v1*v2
            }
            ).sum::<i32>()
        }
    }

    sum

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
        let answer = part_1("files/day3_input.txt");
        assert!(answer == 181345830);
    }

    #[test]
    fn test_part_2_test_input() {
        let answer = part_2("files/test2_input.txt");
        assert!(answer == 48);
    }

    #[test]
    fn test_part_2_real_input() {
        let _answer = part_2("files/day3_input.txt");
        // assert!(answer == 181345830);
    }
}

