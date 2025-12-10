use crate::dayrunner::day01::day01;
use crate::dayrunner::day02::day02;
use crate::dayrunner::day03::day03;
use crate::dayrunner::day04::day04;
use crate::dayrunner::day05::day05;
use crate::dayrunner::day06::day06;
use crate::dayrunner::day07::day07;
use crate::dayrunner::day08::day08;
use crate::dayrunner::day09::day09;
use std::path::PathBuf;
use std::time::Instant;

pub fn rundays(path: &str, runtype: &str, day: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut path_buf = PathBuf::from(path);
    path_buf.push(format!("day{:02}", day));
    path_buf.push(format!("{}.txt", runtype));
    let now = Instant::now();
    let result = match day {
        1 => day01::solve(path_buf.to_str().unwrap()),
        2 => day02::solve(path_buf.to_str().unwrap()),
        3 => day03::solve(path_buf.to_str().unwrap()),
        4 => day04::solve(path_buf.to_str().unwrap()),
        5 => day05::solve(path_buf.to_str().unwrap()),
        6 => day06::solve(path_buf.to_str().unwrap()),
        7 => day07::solve(path_buf.to_str().unwrap()),
        8 => day08::solve(path_buf.to_str().unwrap()),
        9 => day09::solve(path_buf.to_str().unwrap()),
        _ => {
            println!("Day {} not implemented yet", day);
            return Err("Day not implemented yet".into());
        }
    };
    let elapsed_time = now.elapsed();
    println!("Took {} seconds.", elapsed_time.as_millis() as f64 / 1000.0);
    result
}
