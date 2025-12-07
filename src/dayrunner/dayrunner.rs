use crate::dayrunner::day01::day01;
use crate::dayrunner::day02::day02;
use crate::dayrunner::day03::day03;
use crate::dayrunner::day04::day04;
use crate::dayrunner::day05::day05;
use std::path::PathBuf;

pub fn rundays(path: &str, runtype: &str, day: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut path_buf = PathBuf::from(path);
    path_buf.push(format!("day{:02}", day));
    path_buf.push(format!("{}.txt", runtype));

    match day {
        1 => day01::solve(path_buf.to_str().unwrap()),
        2 => day02::solve(path_buf.to_str().unwrap()),
        3 => day03::solve(path_buf.to_str().unwrap()),
        4 => day04::solve(path_buf.to_str().unwrap()),
        5 => day05::solve(path_buf.to_str().unwrap()),
        _ => {
            println!("Day {} not implemented yet", day);
            return Err("Day not implemented yet".into());
        }
    }
}
