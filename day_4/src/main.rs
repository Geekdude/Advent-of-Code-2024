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

#[derive(Debug, EnumIter)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug)]
enum ToMatch {
    XMAS,
    MAS,
    AS,
    S,
}

struct Location {
    row: i32,
    col: i32,
}

impl Location {
    fn next_location(&self, direction: &Direction) -> Location{
        match direction {
            Direction::North => Location {row: self.row-1, col: self.col},
            Direction::South => Location {row: self.row+1, col: self.col},
            Direction::East => Location {row: self.row, col: self.col+1},
            Direction::West => Location {row: self.row, col: self.col-1},
            Direction::NorthEast => Location {row: self.row-1, col: self.col+1},
            Direction::NorthWest => Location {row: self.row-1, col: self.col-1},
            Direction::SouthEast => Location {row: self.row+1, col: self.col+1},
            Direction::SouthWest => Location {row: self.row+1, col: self.col-1},
        }
    }
}

struct Puzzle (Vec<Vec<char>>);

impl Puzzle {
    /// Determine if the location is in bounds.
    fn in_bounds(&self, location: &Location) -> bool {
        location.row >= 0 && location.row < self.num_rows().try_into().unwrap() && location.col >= 0 && location.col < self.num_cols().try_into().unwrap()
    }

    fn get(&self, location: &Location) -> char {
        let Location {row: i, col: j} = location;
        self.0[*i as usize][*j as usize]
    }

    /// Get the number of rows.
    fn num_rows (&self) -> usize { self.0.len() }

    /// Get the number of rows.
    fn num_cols (&self) -> usize { self.0.first().unwrap().len() }
}

fn part_1 (filename: &str) -> i32 {

    let contents = read_from_file(filename);

    let puzzle: Puzzle = Puzzle(contents.lines().map(|l| l.chars().collect()).collect());

    let value = calculate_value_part_1(&puzzle);

    println!("Part 1 Solution: {value}");
    value
}

fn part_2 (filename: &str) -> i32 {

    let contents = read_from_file(filename);

    let puzzle: Puzzle = Puzzle(contents.lines().map(|l| l.chars().collect()).collect());

    let value = calculate_value_part_2(&puzzle);

    println!("Part 2 Solution: {value}");
    value
}

fn read_from_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Unable to read file.")
}

fn calculate_value_part_1(puzzle: &Puzzle) -> i32 {
    let mut count = 0;
    for r in 0..puzzle.num_rows() {
        for c in 0..puzzle.num_cols() {
            for d in Direction::iter() {
                if is_xmas(
                    puzzle, 
                    ToMatch::XMAS, 
                    Location{row:r.try_into().unwrap(),col:c.try_into().unwrap()},
                    &d) {count += 1}
            }
        }
    }

    count
}

fn calculate_value_part_2(puzzle: &Puzzle) -> i32 {
    let mut count = 0;
    for r in 1..puzzle.num_rows()-1 {
        for c in 1..puzzle.num_cols()-1 {
            let loc = Location {row: r as i32, col: c as i32};

            if puzzle.get(&loc) != 'A' {continue;}

            let loc_nw = loc.next_location(&Direction::NorthWest);
            let loc_se = loc.next_location(&Direction::SouthEast);
            let loc_ne = loc.next_location(&Direction::NorthEast);
            let loc_sw = loc.next_location(&Direction::SouthWest);

            let char_nw = puzzle.get(&loc_nw);
            let char_se = puzzle.get(&loc_se);
            let char_ne = puzzle.get(&loc_ne);
            let char_sw = puzzle.get(&loc_sw);

            if ((char_nw == 'M' && char_se == 'S') || (char_nw == 'S' && char_se == 'M')) && ((char_ne == 'M' && char_sw == 'S') || (char_ne == 'S' && char_sw == 'M')) {count += 1}
        }
    }

    count
}

fn is_xmas(puzzle: &Puzzle, remaining: ToMatch, location: Location, direction: &Direction) -> bool {

    // Return false if out of bounds.
    if !puzzle.in_bounds(&location) {return false;}
     
    // Return falues if leter does not match.
    if puzzle.get(&location) != match remaining {
        ToMatch::XMAS => 'X',
        ToMatch::MAS => 'M',
        ToMatch::AS => 'A',
        ToMatch::S => 'S',
    } {return false;}


    // Full word matched.
    if let ToMatch::S = remaining {
        return true;
    }

    // Recursivly try to find if match.
    {
        let next_remaining = match remaining {
            ToMatch::XMAS => ToMatch::MAS,
            ToMatch::MAS => ToMatch::AS,
            ToMatch::AS => ToMatch::S,
            _ => panic!("Unexpected State."),
        };

        let next_location = location.next_location(direction);

        is_xmas(puzzle, next_remaining, next_location, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_test_input() {
        let answer = part_1("files/test_input.txt");
        assert!(answer == 18);
    }

    #[test]
    fn test_part_1_real_input() {
        let answer = part_1("files/day_4.txt");
        assert!(answer == 2397);
    }

    #[test]
    fn test_part_2_test_input() {
        let answer = part_2("files/test_input.txt");
        assert!(answer == 9);
    }

    #[test]
    fn test_part_2_real_input() {
        let _answer = part_2("files/day_4.txt");
        // assert!(answer == 1824);
    }
}

