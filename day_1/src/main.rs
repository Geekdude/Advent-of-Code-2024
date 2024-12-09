use std::io;
use regex::Regex;
use itertools::izip;
use std::collections::HashMap;

fn main() {
    // Read in two lists of numbers.
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    let mut h2: HashMap<i32, i32> = HashMap::new();
    let re = Regex::new(r"^(\d+)\s+(\d+)$").unwrap();

    loop {
        let mut read_string = String::new();

        io::stdin()
            .read_line(&mut read_string).expect("io read error");

        let Some(cap,) = re.captures(&read_string.trim()) else {
            break;
        };

        let number1 = &cap[1];
        let number2 = &cap[2];

        let number1:i32 = number1.parse().unwrap();
        let number2:i32 = number2.parse().unwrap();

        v1.push(number1);
        v2.push(number2);

        let count = h2.entry(number2).or_insert(0);
        *count += 1; 
    }

    // Sort both lists
    v1.sort();
    v2.sort();

    // Compare distance of both numbers (Part 1)
    let mut distance: i32 = 0;
    for (x, y) in izip!(&v1, &v2) {
        let d = (x - y).abs();
        // println!("dist({x},{y})={d}");
        distance += d;
    }

    println!("Part 1 Distance: {distance}");

    // Calculate Part 2 Distance
    let mut distance_p2: i32 = 0;
    for x in v1 {
        let count = h2.entry(x).or_insert(0);
        let d = x * *count;
        // println!("dist({x},hash{x}={count})={d}");
        distance_p2 += d;
    }

    println!("Part 2 Distance: {distance_p2}");

}