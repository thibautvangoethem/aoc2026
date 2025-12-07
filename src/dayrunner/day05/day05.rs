use std::{cmp, fs, str::FromStr};
pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}
#[derive(Debug, Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}
struct ParseError;
impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let vals: Vec<&str> = s.trim().split("-").collect();
        assert!(vals.len() == 2);
        let mut iter = vals.iter();
        let val1 = iter.next().to_owned().unwrap();
        let val2 = iter.next().to_owned().unwrap();
        Ok(Range {
            start: val1.parse().unwrap(),
            end: val2.parse().unwrap(),
        })
    }
}

fn contains_in_range(id: u64, ranges: &Vec<Range>) -> bool {
    for range in ranges {
        if id >= range.start && id <= range.end {
            return true;
        }
    }
    false
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.lines();

    let mut ranges: Vec<Range> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut parseids = false;
    for line in contents {
        if line.is_empty() {
            parseids = true;
            continue;
        }
        if parseids {
            ids.push(match u64::from_str(line) {
                Ok(val) => val,
                Err(_error) => panic!("Could not parse: {line:?}"),
            });
        } else {
            ranges.push(match Range::from_str(line) {
                Ok(val) => val,
                Err(_error) => panic!("Could not parse: {line:?}"),
            });
        }
    }
    let count = ids
        .iter()
        .filter(|&val| contains_in_range(*val, &ranges))
        .count();
    println!("the result for day05:1 is {0:?}", count);
    Ok(())
}

// extra func needed as both ids need to be in a single range
fn contains_both_in_range(id1: u64, id2: u64, ranges: &Vec<Range>) -> bool {
    for range in ranges {
        if id1 >= range.start && id1 <= range.end && id2 >= range.start && id2 <= range.end {
            return true;
        }
    }
    false
}

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut hasmerged = true;
    let mut itervec = ranges;
    let mut newvec: Vec<Range> = Vec::new();
    while hasmerged {
        hasmerged = false;
        for (idx, handling_range) in itervec.iter().enumerate() {
            //Case this range is already encapsulated and can be skipped
            if contains_both_in_range(handling_range.start, handling_range.end, &newvec) {
                continue;
            };
            //Case range could be merged with other vec
            let mut found = false;
            for iter_range in itervec.iter().skip(idx + 1) {
                if handling_range.start <= iter_range.end && iter_range.start <= handling_range.end
                {
                    found = true;
                    hasmerged = true;
                    newvec.push(Range {
                        start: cmp::min(handling_range.start, iter_range.start),
                        end: cmp::max(handling_range.end, iter_range.end),
                    });
                    break;
                }
            }
            //Case range could not be merged
            if !found {
                newvec.push(handling_range.clone());
            }
        }
        itervec = newvec.clone();
        newvec = Vec::new();
    }

    return itervec;
}

pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.lines();

    let mut ranges: Vec<Range> = Vec::new();
    for line in contents {
        if line.is_empty() {
            break;
        }
        ranges.push(match Range::from_str(line) {
            Ok(val) => val,
            Err(_error) => panic!("Could not parse: {line:?}"),
        });
    }

    let count: u64 = merge_ranges(ranges)
        .iter()
        .map(|range| (range.end - range.start) + 1)
        .sum();

    println!("the result for day05:2 is {0:?}", count);
    Ok(())
}
