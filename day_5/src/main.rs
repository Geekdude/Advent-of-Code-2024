use clap::Parser;
use std::{collections::HashMap, fs};
use regex::Regex;
use petgraph::{graph::{DiGraph, NodeIndex}, Graph};
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

fn part_1 (filename: &str) -> u32 {

    let (print_rules, print_jobs) = read_from_file(filename);

    let value = calculate_value_part_1(&print_rules, &print_jobs);

    println!("Part 1 Solution: {value}");
    value
}

fn part_2 (filename: &str) -> u32 {

    let (print_rules, mut print_jobs) = read_from_file(filename);

    let value = calculate_value_part_2(&print_rules, &mut print_jobs);

    println!("Part 2 Solution: {value}");
    value
}

#[derive(Debug)]
struct PrintRules {
    nodes: HashMap<u32,NodeIndex>,
    rules: DiGraph<u32, ()>,
}

impl PrintRules {
    fn new () -> PrintRules {
        Self { nodes: HashMap::new(), rules: Graph::new() }
    }

    fn get_or_add_node(&mut self, node_id: u32) -> NodeIndex {
         if let std::collections::hash_map::Entry::Vacant(e) = self.nodes.entry(node_id) {                
            let node = self.rules.add_node(node_id);                       
            e.insert(node);
            node
        } else{
            *self.nodes.get(&node_id).unwrap()
        }                                         
    }

    fn valid(&self, a: u32, b: u32) -> bool {
        let Some(a_n) = self.nodes.get(&a) else {return true};
        let Some(b_n) = self.nodes.get(&b) else {return true};

        !self.rules.contains_edge(*b_n, *a_n)
    }
}

struct PrintJobs {
    jobs: Vec<Vec<u32>>,
}

impl PrintJobs {
    fn new () -> PrintJobs {
        Self {jobs: Vec::new() }
    }
}


fn read_from_file(filename: &str) -> (PrintRules, PrintJobs) {
    let contents = fs::read_to_string(filename).expect("Unable to read file.");

    let re = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
    let mut print_rules = PrintRules::new();
    let mut print_jobs = PrintJobs::new();

    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(cap) = re.captures(trimmed) {
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();
            let x = print_rules.get_or_add_node(x);
            let y = print_rules.get_or_add_node(y);
            print_rules.rules.add_edge(x, y, ());
        }
        else if !trimmed.is_empty() {
            print_jobs.jobs.push(trimmed.split(',').map(|x| x.parse().unwrap()).collect());                        
        }
    }

    // println!("{print_rules:#?}");

    (print_rules, print_jobs)
}

fn calculate_value_part_1(print_rules: &PrintRules, print_jobs: &PrintJobs) -> u32 {
    let mut count = 0;
    for i in 0..print_jobs.jobs.len() {
        let valid = print_jobs.jobs[i].clone().into_iter().combinations(2).map(|r| print_rules.valid(r[0], r[1])).all(|x| x);

        if valid {
            let middle = print_jobs.jobs[i].len()/2;
            count += print_jobs.jobs[i][middle];
        }

    }
    count
}

fn calculate_value_part_2(print_rules: &PrintRules, print_jobs: &mut PrintJobs) -> u32 {
    let mut count = 0;
    for i in 0..print_jobs.jobs.len() {

        let valid = print_jobs.jobs[i].clone().into_iter().combinations(2).map(|r| print_rules.valid(r[0], r[1])).all(|x| x);

        if !valid {
            for j in 0..print_jobs.jobs[i].len() {
                for k in j+1..print_jobs.jobs[i].len() {
                    let a = print_jobs.jobs[i][j];
                    let b = print_jobs.jobs[i][k];
                    if !print_rules.valid(a, b) {
                        print_jobs.jobs[i].swap(j, k);
                    }

                }
            }
            let middle = print_jobs.jobs[i].len()/2;
            count += print_jobs.jobs[i][middle];
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
        assert!(answer == 143);
    }

    #[test]
    fn test_part_1_real_input() {
        let answer = part_1("files/day_5.txt");
        assert!(answer == 4135);
    }

    #[test]
    fn test_part_2_test_input() {
        let answer = part_2("files/test_input.txt");
        assert!(answer == 123);
    }

    #[test]
    fn test_part_2_real_input() {
        let answer = part_2("files/day_5.txt");
        assert!(answer == 5285);
    }
}

