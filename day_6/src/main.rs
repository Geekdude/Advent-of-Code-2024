use clap::Parser;
use std::{char, collections::HashSet, fs};

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

#[derive(Debug, PartialEq, Copy, Clone)]
enum CellType {
    Unvisited,
    Visited,
    Obstacle,
    Guard,
}

impl CellType {
    /// Convert a char to a cell type.
    fn from_char (c: char) -> CellType {
        match c {
           '.' => CellType::Unvisited,
           'X' => CellType::Visited,
           '#' => CellType::Obstacle,
           '^' => CellType::Guard,
           v => panic!("Unknown cell type {v}.") 
        }
    }

    // /// Convert a cell type to a char.
    // fn as_char (&self) -> char {
    //     match self {
    //         Self::Unvisited => '.',
    //         Self::Visited => 'X',
    //         Self::Obstacle => '#',
    //         Self::Guard => '^',
    //     }
    // }
}

#[derive(Debug, Clone)]
struct Map {
    map :Vec<Vec<CellType>>,
}

impl Map {
    fn get_location(&self, loc: &Location) -> Option<&CellType> {
        let row = usize::try_from(loc.row).ok()?;
        let col = usize::try_from(loc.col).ok()?;
        self.map.get(row)?.get(col)
    }

    fn visit_location(&mut self, loc: &Location) {
        let row = usize::try_from(loc.row).unwrap();
        let col = usize::try_from(loc.col).unwrap();
        assert!(self.map[row][col] == CellType::Unvisited);
        self.map[row][col] = CellType::Visited;
    }

    fn add_obstacle(&mut self, loc: &Location) {
        let row = usize::try_from(loc.row).unwrap();
        let col = usize::try_from(loc.col).unwrap();
        assert!(self.map[row][col] == CellType::Unvisited);
        self.map[row][col] = CellType::Obstacle;
    }

    fn from_string(content: &str) -> (Self, Guard)  {
        let mut guard  = Guard::new(Direction::North, 0, 0);

        let mut map = Self {map: content.lines()
            .map(|line| line.chars()
            .map(CellType::from_char).collect()).collect()
        };

        for i in 0..map.map.len() {
            for j in 0..map.map[i].len() {
                if map.map[i][j] == CellType::Guard {
                    map.map[i][j] = CellType::Unvisited;
                    guard.location = Location{row: i as i32, col: j as i32};
                }
            }
        }
    
        (map, guard)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Guard {
    direction: Direction,
    location: Location,
}

#[derive(Debug, PartialEq)]
enum MoveResult {
    WalkForward,
    Turn,
    Exit,
}

impl Guard {
    /// Create a new Guard.
    fn new(facing: Direction, row: i32, col: i32) -> Self {
        Self {direction: facing, location: Location {row, col}}
    }

    fn turn_guard(&mut self) {
        self.direction = match self.direction {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }
    
    fn move_guard(&mut self, map: &mut Map) -> MoveResult {
        let next_location = self.location.next_location(&self.direction);
        let next_cell = map.get_location(&next_location);

        match next_cell {
            Some(next) => match next {
                CellType::Obstacle => {
                    self.turn_guard(); 
                    MoveResult::Turn
                },
                CellType::Visited => {
                    self.location = next_location; 
                    MoveResult::WalkForward
                },
                CellType::Unvisited => {
                    self.location = next_location; 
                    map.visit_location(&self.location); 
                    MoveResult::WalkForward
                },
                CellType::Guard => {
                    panic!("Unexpected cell type.")
                }
            }
            None => {
                MoveResult::Exit
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        }
    }
}

fn calculate_value_part_1(contents: &str) -> i32 {
    let (mut map, mut guard) = Map::from_string(contents);


    loop {
        let result = guard.move_guard(&mut map);
        if result == MoveResult::Exit {break}
    }

    let mut count = 0;
    for v in map.map {
        for i in v {
            if i == CellType::Visited {
                count += 1;
            }
        }
    }

    count
}

fn calculate_value_part_2(contents: &str) -> i32 {
    let (map, guard) = Map::from_string(contents);

    let guard_next_loc = guard.location.next_location(&Direction::North);

    let mut count = 0;

    for i in 0..map.map.len() {
        for j in 0..map.map[i].len() {
            if map.map[i][j] == CellType::Unvisited && !(i == guard_next_loc.row as usize && j == guard_next_loc.col as usize ) {
                let mut map_ut = map.clone();
                let mut guard_ut = guard.clone();
                map_ut.add_obstacle(&Location { row: i as i32, col: j as i32 });

                let mut loop_check = HashSet::new(); 

                loop {
                    let result = guard_ut.move_guard(&mut map_ut);
                    if result == MoveResult::Exit {break}
                    if loop_check.contains(&guard_ut) {
                        count += 1;
                        break;
                    }
                    else {
                        loop_check.insert(guard_ut.clone());
                    }
                }
            }
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
        assert!(answer == 41);
    }

    #[test]
    fn test_part_1_real_input() {
        let answer = part_1("files/day_6.txt");
        assert!(answer == 4964);
    }

    #[test]
    fn test_part_2_test_input() {
        let answer = part_2("files/test_input.txt");
        assert!(answer == 6);
    }

    #[test]
    fn test_part_2_real_input() {
        let answer = part_2("files/day_6.txt");
        assert!(answer == 1740);
    }
}

