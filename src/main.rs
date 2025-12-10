mod dayrunner;
mod generator;

use dayrunner::dayrunner::rundays;
use generator::generator::generate;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mode, day, runtype) = parse_args()?;
    //let (mode, day, runtype) = ("run".to_string(), 9, Some("test".to_string()));
    run_mode(mode, day, runtype)
}

fn parse_args() -> Result<(String, u32, Option<String>), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day> [mode] [-generate]", args[0]);
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().expect("Day must be a number");
    let mode = if args.len() > 3 && args[3] == "-generate" {
        "generate".to_string()
    } else {
        "run".to_string()
    };
    let runtype: Option<String> = if args.len() > 2 && args[2] != "-generate" {
        Some(args[2].clone())
    } else {
        None
    };

    Ok((mode, day, runtype))
}

fn run_mode(
    mode: String,
    day: u32,
    runtype: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    match mode.as_str() {
        "generate" => generate(r"C:\Users\thiba\Desktop\aoc\aoc2025\src\dayrunner", day),
        "run" => {
            let runtype = runtype.unwrap();
            if day == 0 {
                for i in 1..=12 {
                    let _ = rundays(r"C:\Users\thiba\Desktop\aoc\aoc2025\input", &runtype, i);
                }
                Ok(())
            } else {
                rundays(r"C:\Users\thiba\Desktop\aoc\aoc2025\input", &runtype, day)
            }
        }
        _ => {
            eprintln!("Invalid mode. Use 'generate' or 'run'.");
            std::process::exit(1);
        }
    }
}
