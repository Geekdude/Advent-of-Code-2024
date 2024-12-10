use std::{cmp::Ordering, io};
use itertools::Itertools;
use tracing::{event, Level};
use tracing_subscriber;

#[derive(Debug)]
enum Mode {
    Increasing,
    Decreasing,
    Unset,
}

fn main() {

    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(Level::ERROR).finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("Error registering tracer.");
    

    // Read in matrix of numbers.
    let reports: Vec<Vec<i32>> = io::stdin().lines()
        .map(|l| l.unwrap().trim().split_whitespace()
        .map(|number| number.parse().unwrap()).collect())
        .collect();

    let mut safe_count_pt1 = 0;
    let mut safe_count_pt2 = 0;

    // Determine if the report is safe or unsafe
    for report in &reports {

        event!{Level::INFO, "Report {report:?}"};

        let mut safe = true;
        let mut mode = Mode::Unset;

        for (x, y) in report.into_iter().tuple_windows() {
            let delta = x - y;

            event!{Level::INFO, "x {x} y {y}"};
            if let Mode::Unset = mode {
                match x.cmp(&y) {
                    Ordering::Equal => { safe = false; break } ,
                    Ordering::Less => mode = Mode::Increasing,
                    Ordering::Greater => mode = Mode::Decreasing,
                }
            }

            event!(Level::INFO, "Mode = {mode:?}, delta = {delta}");
            
            match mode {
                Mode::Increasing => {if !(delta <= -1 && delta >= -3) {safe = false; break} },
                Mode::Decreasing => {if !(delta >= 1 && delta <= 3) {safe = false; break} },
                Mode::Unset => {panic!("Unexpected state.")}
            }
        }

        if safe == true {
            event!(Level::INFO, "Report {report:?} is safe");
            safe_count_pt1 += 1;
        }
    }

    // Determine if the report is safe or unsafe
    for report in &reports {

        event!{Level::INFO, "Report {report:?}"};

        // For each removal point
        for i in 0..report.len() {
            let mut report_with_drop = report.to_vec();
            report_with_drop.remove(i);

            let mut safe = true;
            let mut mode = Mode::Unset;

            for (x, y) in report_with_drop.into_iter().tuple_windows() {
                let delta = x - y;

                event!{Level::INFO, "x {x} y {y}"};
                if let Mode::Unset = mode {
                    match x.cmp(&y) {
                        Ordering::Equal => { safe = false; break } ,
                        Ordering::Less => mode = Mode::Increasing,
                        Ordering::Greater => mode = Mode::Decreasing,
                    }
                }

                event!(Level::INFO, "Mode = {mode:?}, delta = {delta}");
                
                match mode {
                    Mode::Increasing => {if !(delta <= -1 && delta >= -3) {safe = false; break} },
                    Mode::Decreasing => {if !(delta >= 1 && delta <= 3) {safe = false; break} },
                    Mode::Unset => {panic!("Unexpected state.")}
                }
            }

            if safe == true {
                event!(Level::INFO, "Report {report:?} is safe");
                safe_count_pt2 += 1;
                break;
            }
        }
    }

    println!("Safe count part 1 = {safe_count_pt1}");
    println!("Safe count part 2 = {safe_count_pt2}");
}
