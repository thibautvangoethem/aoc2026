use crate::dayrunner::day01::day01;
use std::path::PathBuf;

pub fn rundays(path: &str, runtype: &str, day: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut path_buf = PathBuf::from(path);
    path_buf.push(format!("day{:02}", day));
    path_buf.push(format!("{}.txt", runtype));

    match day {
        1 => day01::solve(path_buf.to_str().unwrap()),
        // 2 => day02::solve1(path_buf.to_str().unwrap()),
        // Add more days as needed
        _ => {
            println!("Day {} not implemented yet", day);
            return Err("Day not implemented yet".into());
        }
    }
}
