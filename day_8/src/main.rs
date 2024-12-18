use clap::Parser;
use std::{collections::{HashMap, HashSet}, fs};
use itertools::Itertools;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Antenna {
    id: char, 
    location: Location,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    row: i32,
    col: i32,
}

struct Map {
    rows: usize,
    cols: usize,
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

fn verify_location(loc: &Location, map: &Map) -> Option<Location> {

    if loc.row >= 0 && loc.row < map.rows as i32 && loc.col >= 0 && loc.col < map.cols as i32 {
        Some(loc.clone())
    }
    else {
        None
    }

}

fn calculate_antinodes(loc1: &Location, loc2: &Location) -> (Location, Location) {
    let delta_y = loc2.row-loc1.row;
    let delta_x =  loc2.col-loc1.col;

    let antinode1 = Location{row: loc2.row+delta_y, col: loc2.col+delta_x};
    let antinode2 = Location{row: loc1.row-delta_y, col: loc1.col-delta_x};

    (antinode1, antinode2)
}

fn find_resonant(loc1: &Location, loc2: &Location, map: &Map) -> Vec<Location> {
    let mut rtn = Vec::new();

    let delta_y = loc2.row-loc1.row;
    let delta_x = loc2.col-loc1.col;

    rtn.push(loc1.clone());
    rtn.push(loc2.clone());

    let mut next = loc2.clone();
    loop {
        next.row = &next.row+delta_y;
        next.col =  &next.col+delta_x;
        let nextv = verify_location(&next, &map);
        match nextv {
            Some(loc) => rtn.push(loc.clone()),
            None => break,
        }
    }
    let mut next = loc1.clone();
    loop {
        next.row = &next.row-delta_y;
        next.col =  &next.col-delta_x;
        let nextv = verify_location(&next, &map);
        match nextv {
            Some(loc) => rtn.push(loc.clone()),
            None => break,
        }
    }

    rtn
}

fn calculate_value_part_1(contents: &str) -> i32 {
    let mut map = Map{rows: 0, cols: 0};

    let mut antennas = HashMap::new();

    for (row, line) in contents.lines().enumerate() {
        if row+1 > map.rows {
            map.rows = row+1
        }
        for (col, char) in line.chars().enumerate() {
            if col+1 > map.cols {
                map.cols = col+1
            }
            match char {
                '.' => {},
                x => {
                    let antenna = Antenna{id: x, location: Location{row: row as i32, col: col as i32}};
                    let item = antennas.entry(char).or_insert(vec![]);
                    item.push(antenna);
                }
            }
        }
    }

    let mut antinodes = HashSet::new();

    for (_channel, antennas) in antennas.iter() {
        for comb in antennas.iter().combinations(2) {
            let a1 = comb[0];
            let a2 = comb[1];
            let (an1, an2) = calculate_antinodes(&a1.location, &a2.location);
            let an1 = verify_location(&an1, &map);
            let an2 = verify_location(&an2, &map);

            if let Some(a) = an1 {antinodes.insert(a);}
            if let Some(a) = an2 {antinodes.insert(a);}
        }
    }

    antinodes.len() as i32
}

fn calculate_value_part_2(contents: &str) -> i32 {
    let mut map = Map{rows: 0, cols: 0};

    let mut antennas = HashMap::new();

    for (row, line) in contents.lines().enumerate() {
        if row+1 > map.rows {
            map.rows = row+1
        }
        for (col, char) in line.chars().enumerate() {
            if col+1 > map.cols {
                map.cols = col+1
            }
            match char {
                '.' => {},
                x => {
                    let antenna = Antenna{id: x, location: Location{row: row as i32, col: col as i32}};
                    let item = antennas.entry(char).or_insert(vec![]);
                    item.push(antenna);
                }
            }
        }
    }

    let mut antinodes = HashSet::new();

    for (_channel, antennas) in antennas.iter() {
        for comb in antennas.iter().combinations(2) {
            let a1 = comb[0];
            let a2 = comb[1];
            let res = find_resonant(&a1.location, &a2.location, &map);
            for r in res {
                antinodes.insert(r);
            }
        }
    }

    antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_test_input() {
        let answer = part_1("files/test_input.txt");
        let correct = 14;
        assert!(answer == correct, "Answer is {answer} not {correct}.");
    }

    #[test]
    fn test_part_1_real_input() {
        let answer = part_1("files/day_8.txt");
        let correct = 413;
        assert!(answer == correct, "Answer is {answer} not {correct}.");
    }

    #[test]
    fn test_part_2_test_input() {
        let answer = part_2("files/test_input.txt");
        let correct = 34;
        assert!(answer == correct, "Answer is {answer} not {correct}.");
    }

    #[test]
    fn test_part_2_real_input() {
        let answer = part_2("files/day_8.txt");
        let correct = 1417;
        assert!(answer == correct, "Answer is {answer} not {correct}.");
    }
}

