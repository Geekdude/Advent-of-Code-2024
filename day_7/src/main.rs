use clap::Parser;
use std::fs;
use strum::{EnumIter, IntoEnumIterator};

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

fn part_1 (filename: &str) -> u64 {

    let contents = read_from_file(filename);

    let value = calculate_value_part_1(&contents);

    println!("Part 1 Solution: {value}");
    value
}

fn part_2 (filename: &str) -> u64 {

    let contents = read_from_file(filename);

    let value = calculate_value_part_2(&contents);

    println!("Part 2 Solution: {value}");
    value
}

fn read_from_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Unable to read file.")
}

struct Operators {
    operators: Vec<Operator>
}

impl Operators {
    fn new(content: &str) -> Operators {
        let operators = content.lines().map(|line| {
            let mut split = line.split(':');
            let test_input = split.next().unwrap().parse().unwrap();
            let values = split.next().unwrap().split_whitespace().map(|w| w.parse::<u64>().unwrap()).collect();
            Operator{test_value: test_input, numbers: values}
        }).collect();

        Operators{operators}
    }
    fn iter(&self) -> impl Iterator<Item = &Operator> {
        self.operators.iter()
    }
}

struct Operator {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Operator {
    fn valid_test1(&self) -> bool {
        Operator::is_valid1(self.test_value, &self.numbers[1..], self.numbers[0])
    }

    fn valid_test2(&self) -> bool {
        Operator::is_valid2(self.test_value, &self.numbers[1..], self.numbers[0])
    }

    fn is_valid1(test_value: u64, values: &[u64], partial: u64) -> bool {
        if values.is_empty() {
            return test_value == partial;
        }

        for op in Operations::iter() {
            let partial = match op {
                Operations::Add => partial + values.first().unwrap(),
                Operations::Multiply => partial * values.first().unwrap(),
                Operations::Concat => continue,
            };

            let result = Operator::is_valid1(test_value, &values[1..], partial);

            if result {
                return true;
            }
            
        }
        false
    }
    fn is_valid2(test_value: u64, values: &[u64], partial: u64) -> bool {
        if values.is_empty() {
            return test_value == partial;
        }

        for op in Operations::iter() {
            let partial = match op {
                Operations::Add => partial + values.first().unwrap(),
                Operations::Multiply => partial * values.first().unwrap(),
                Operations::Concat => (partial.to_string() + &values.first().unwrap().to_string()).parse().unwrap(),
            };

            let result = Operator::is_valid2(test_value, &values[1..], partial);

            if result {
                return true;
            }
        }
        false

    }
}

#[derive(Debug, EnumIter)]
enum Operations {
    Add,
    Multiply,
    Concat,
}

fn calculate_value_part_1(contents: &str) -> u64 {
    let operators = Operators::new(contents);
    
    let mut count: u64 = 0;
    for operator in operators.iter() {
        if operator.valid_test1() {
            count += operator.test_value;
        }
    }
    count
}

fn calculate_value_part_2(contents: &str) -> u64 {
    let operators = Operators::new(contents);
    
    let mut count: u64 = 0;
    for operator in operators.iter() {
        if operator.valid_test2() {
            count += operator.test_value;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_test_input() {
        let answer = part_1("files/test_input.txt");
        let correct = 3749;
        assert!(answer == correct, "Answer is {answer} not {correct}.");
    }

    #[test]
    fn test_part_1_real_input() {
        let answer = part_1("files/day_7.txt");
        let correct = 21572148763543;
        assert!(answer == correct, "Answer is {answer} not {correct}.");
    }

    #[test]
    fn test_part_2_test_input() {
        let answer = part_2("files/test_input.txt");
        let correct = 11387;
        assert!(answer == correct, "Answer is {answer} not {correct}.");
    }

    #[test]
    fn test_part_2_real_input() {
        let answer = part_2("files/day_7.txt");
        let correct = 581941094529163;
        assert!(answer == correct, "Answer is {answer} not {correct}.");
    }
}

