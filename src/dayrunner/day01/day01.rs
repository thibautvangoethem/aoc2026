use std::{fs, str::FromStr};

pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

struct SingleMove {
    way: String,
    amount: i32,
}

struct ParseError;
impl FromStr for SingleMove {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let (x, y) = s.trim().split_at(1);
        match y.parse::<i32>() {
            Ok(value) => Ok(SingleMove {
                way: x.to_string(),
                amount: value,
            }),
            Err(_e) => Err(ParseError),
        }
    }
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.split("\n");

    let mut dial: i32 = 50;
    let mut counter: i32 = 0;
    for line in contents {
        let parsed_line = match SingleMove::from_str(line) {
            Ok(val) => val,
            Err(_error) => panic!("Could not parse: {line:?}"),
        };
        match parsed_line.way.as_str() {
            "L" => dial -= parsed_line.amount,
            "R" => dial += parsed_line.amount,
            _ => panic!("Unknow route: {0:?}", parsed_line.way),
        }
        while dial < 0 || dial > 99 {
            if dial < 0 {
                dial += 100
            }
            if dial > 99 {
                dial -= 100
            }
        }
        // println!("the value of the dial is {0:?}", dial);
        if dial == 0 {
            counter += 1
        };
    }
    println!("the result for day01:1 is {0:?}", counter);

    Ok(())
}
pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.split("\n");

    let mut dial: i32 = 50;
    let mut counter: i32 = 0;
    for line in contents {
        let parsed_line = match SingleMove::from_str(line) {
            Ok(val) => val,
            Err(_error) => panic!("Could not parse: {line:?}"),
        };
        // println!("action {0:?}{1:?}", parsed_line.way, parsed_line.amount);
        match parsed_line.way.as_str() {
            "L" => {
                //case where you ended on 0 before and move left again, avoids double counting
                if dial == 0 {
                    counter -= 1
                }
                dial -= parsed_line.amount;
                while dial < 0 {
                    dial += 100;
                    counter += 1;
                    // println!("click");
                }
                //case end on 0
                if dial == 0 {
                    // println!("click");
                    counter += 1;
                }
            }
            "R" => {
                dial += parsed_line.amount;
                while dial > 99 {
                    dial -= 100;
                    counter += 1;
                    // println!("click")
                }
            }
            _ => panic!("Unknown route: {0:?}", parsed_line.way),
        }
        // println!("the value of the dial is {0:?}", dial);
    }
    println!("the result for day01:2 is {0:?}", counter);

    Ok(())
}
