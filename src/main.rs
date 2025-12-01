mod dayrunner;
mod generator;

use dayrunner::dayrunner::rundays;
use generator::generator::generate;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mode, day, runtype) = parse_args()?;
    // let mode = "run".to_string();
    // let day = 1;
    // let runtype = Some("test".to_string());
    run_mode(mode, day, runtype)
}

fn parse_args() -> Result<(String, u32, Option<String>), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day> [mode] [runtype]", args[0]);
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().expect("Day must be a number");
    let mode = if args.len() > 3 {
        args[3].clone()
    } else {
        "run".to_string()
    };
    let runtype = if args.len() > 2 {
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
        "generate" => generate(r"C:\Users\thiba\Desktop\aoc\aoc2026\src\dayrunner", day),
        "run" => {
            let runtype = runtype.unwrap();
            rundays(r"C:\Users\thiba\Desktop\aoc\aoc2026\input", &runtype, day)
        }
        _ => {
            eprintln!("Invalid mode. Use 'generate' or 'run'.");
            std::process::exit(1);
        }
    }
}
