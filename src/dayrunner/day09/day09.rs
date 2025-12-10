use core::fmt;
use std::{cmp, fs, str::FromStr};

pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
struct ParseError;
impl FromStr for Position {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let vals: Vec<&str> = s.trim().split(",").collect();
        assert!(vals.len() == 2);
        let mut iter = vals.iter();
        let val1 = iter.next().to_owned().unwrap();
        let val2 = iter.next().to_owned().unwrap();
        Ok(Position {
            x: val1.parse().unwrap(),
            y: val2.parse().unwrap(),
        })
    }
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let positions = binding
        .lines()
        .map(|line| Position::from_str(line).unwrap())
        .collect::<Vec<Position>>();
    let mut maxarea = 0;
    for i in 0..positions.len() {
        let pos1 = positions.get(i).unwrap();
        for j in i + 1..positions.len() {
            let pos2 = positions.get(j).unwrap();
            let area = (i64::abs(pos1.x - pos2.x) + 1) * (i64::abs(pos1.y - pos2.y) + 1);
            if area > maxarea {
                maxarea = area
            }
        }
    }
    println!("the result for day09:1 is {0:?}", maxarea);
    Ok(())
}

// fn fillGrid(grid: &mut Vec<Vec<bool>>, pos1: Position, pos2: Position) {
//     if pos1.x == pos2.x {
//         for i in pos1.y..=pos2.y {
//             grid[pos1.x as usize][i as usize] = true;
//         }
//     } else if pos1.y == pos2.y {
//         for i in pos1.x..=pos2.x {
//             grid[i as usize][pos1.y as usize] = true;
//         }
//     } else {
//         panic!("this shouldnt happen");
//     }
// }
#[derive(Debug, Hash, Eq, PartialEq)]
enum Directions {
    N,
    S,
    W,
    E,
}

impl fmt::Display for Directions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Directions::N => write!(f, "N"),
            Directions::S => write!(f, "S"),
            Directions::E => write!(f, "E"),
            Directions::W => write!(f, "W"),
        }
    }
}
fn getDirection(pmain: &Position, pcompare: &Position) -> Directions {
    let xdiff: i64 = pmain.x - pcompare.x;
    let ydiff: i64 = pmain.y - pcompare.y;
    if xdiff != 0 && ydiff != 0 {
        panic!("for this compare 1 dimension should always be the same");
    }

    if xdiff > 0 {
        Directions::W
    } else if xdiff < 0 {
        Directions::E
    } else if ydiff > 0 {
        Directions::N
    } else {
        Directions::S
    }
}

pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let positions = binding
        .lines()
        .map(|line| Position::from_str(line).unwrap())
        .collect::<Vec<Position>>();

    //so a 100000*100000 grid overflows the stack, oops => make vec => 7s init time only. Rust does a lot but not that much
    // let mut grid: Vec<Vec<bool>> = vec![vec![false; 100000]; 100000];
    // for iter in positions.windows(2) {
    //     fillGrid(&mut grid, iter[0], iter[1]);
    // }

    //idea sort pos combinations on area size => from highest to lowest, get all positions () in them
    //Do the connect for those parts, then somehow find infeasbilities as fast as possible
    // So barring any edge cases (ignore these for now), following should always hold:
    // given posi (xi,yi) and posj (xj,xj) where xi>xj and yi>yj.
    // A rectangular is valid if for all points no point pm exists where xi>xm>xj yi>ym>yj (basically a rectangular cant contain another point)
    // logical if xi==xj or yi==xj skip (too low of a area for this)
    // if xi>xj but yi<yj then same algorithm but with xi>xm>xj yj>ym>yi
    // edge case: not considering single width lines passing through messing up everithing
    let mut areas: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..positions.len() {
        let pos1 = positions.get(i).unwrap();
        for j in i + 1..positions.len() {
            let pos2 = positions.get(j).unwrap();
            let area = (i64::abs(pos1.x - pos2.x) + 1) * (i64::abs(pos1.y - pos2.y) + 1);
            areas.push((area, i, j));
        }
    }
    areas.sort_by(|a, b| b.0.cmp(&a.0));
    let mut result: i64 = 0;
    let maxindex = positions.len() - 1;
    'Yay: for handling in areas {
        let pos1 = &positions[handling.1];
        let pos2 = &positions[handling.2];
        // println!("handling at {} and {}", pos1, pos2);
        let xmax = cmp::max(pos1.x, pos2.x);
        let xmin = cmp::min(pos1.x, pos2.x);
        let ymax = cmp::max(pos1.y, pos2.y);
        let ymin = cmp::min(pos1.y, pos2.y);
        if xmax == xmin || ymax == ymin {
            panic!("at this point i really hoped for a match already");
        }

        // note still need to check corners => corners need to enclose => future me this is wrong, doesnt even work in the exmaple
        // so needs to be x - coupled with  | or other way around with |
        //                |                -y                          x- => basically all directions need to be gotten when going through
        //yup this is it, the point the code turns horrendous => and it doesnt even work great
        // I might be able to salvage this, the corners cant have their outside points overlap.
        // P - corner (called SE) has it outside pointing NW, now it cant match with any points either N or W of it
        // |
        // => well that idea works on test data, but not real data ...

        let prevpos1 = match handling.1 == 0 {
            true => maxindex,
            false => handling.1 - 1,
        };

        let nextpos1 = match handling.1 == maxindex {
            true => 0,
            false => handling.1 + 1,
        };
        //Note this gives the actual direction, so if it gives N then we must check if the point is actually south to discard
        let dir1 = getDirection(pos1, &positions[prevpos1]);
        let dir2 = getDirection(pos1, &positions[nextpos1]);

        let xdif = pos1.x - pos2.x;
        let ydif = pos1.y - pos2.y;
        //cant have dir2 south of dir 1
        if (dir1 == Directions::N || dir2 == Directions::N) && ydif > 0 {
            continue 'Yay;
        }
        if (dir1 == Directions::S || dir2 == Directions::S) && ydif < 0 {
            continue 'Yay;
        }
        if (dir1 == Directions::E || dir2 == Directions::E) && xdif > 0 {
            continue 'Yay;
        }
        if (dir1 == Directions::W || dir2 == Directions::W) && xdif < 0 {
            continue 'Yay;
        }

        // Check if there are any positions in our rectangle, that basically is a death sentence for the viability
        for pos in &positions {
            // println!("looking at {}", pos);
            if pos == pos1 || pos == pos2 {
                continue;
            }
            // a position in the middle is over for this handle
            if xmax > pos.x && pos.x > xmin && ymax > pos.y && pos.y > ymin {
                continue 'Yay;
            }

            // if ymax > pos.y && pos.y > ymin {
            //     continue 'Yay;
            // }

            // if xmax==pos.x ||
        }
        result = handling.0;
        break 'Yay;
    }
    println!("the result for day09:2 is {0:?}", result);
    Ok(())
}
