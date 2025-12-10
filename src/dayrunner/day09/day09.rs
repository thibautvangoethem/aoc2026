use core::fmt;
use std::collections::VecDeque;
use std::{
    collections::HashMap,
    fs::{self},
    str::FromStr,
};

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

fn fill_grid(grid: &mut Vec<Vec<bool>>, pos1: &Position, pos2: &Position) {
    if pos1.x == pos2.x {
        for i in pos1.y.min(pos2.y)..=pos1.y.max(pos2.y) {
            grid[pos1.x as usize][i as usize] = true;
        }
    } else if pos1.y == pos2.y {
        for i in pos1.x.min(pos2.x)..=pos1.x.max(pos2.x) {
            grid[i as usize][pos1.y as usize] = true;
        }
    } else {
        panic!("this shouldnt happen");
    }
}

fn flood_fill_grid(grid: &mut Vec<Vec<bool>>, known_good: &Position) {
    let mut stack = VecDeque::from([known_good.clone()]);
    while !stack.is_empty() {
        let pos = stack.pop_back().unwrap();
        // println!("{} {}", pos, grid[pos.x as usize][pos.y as usize]);
        grid[pos.x as usize][pos.y as usize] = true;
        match grid.get((pos.x - 1) as usize) {
            Some(val) => {
                if !val[pos.y as usize] {
                    stack.push_back(Position {
                        x: (pos.x - 1),
                        y: (pos.y),
                    })
                }
            }
            _ => (),
        }

        match grid.get((pos.x + 1) as usize) {
            Some(val) => {
                if !val[pos.y as usize] {
                    stack.push_back(Position {
                        x: (pos.x + 1),
                        y: (pos.y),
                    })
                }
            }
            _ => (),
        }

        match grid[pos.x as usize].get((pos.y - 1) as usize) {
            Some(val) => {
                if !val {
                    stack.push_back(Position {
                        x: (pos.x),
                        y: (pos.y - 1),
                    })
                }
            }
            _ => (),
        }
        match grid[pos.x as usize].get((pos.y + 1) as usize) {
            Some(val) => {
                if !val {
                    stack.push_back(Position {
                        x: (pos.x),
                        y: (pos.y + 1),
                    })
                }
            }
            _ => (),
        }
    }
}

pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let positions = binding
        .lines()
        .map(|line| Position::from_str(line).unwrap())
        .collect::<Vec<Position>>();
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
    // => this idea didnt work out but is immortalized in git

    //idea 3: compress the coordinates, and then just brute force check it
    // So every coordinate should be compressible, a x/y coord that does not have a direct coord next to it can be represented by 3 coords
    // A coord that has one next to it get represented with less coords so they still match
    //If i have x coords 1 1000 1001 5000 i represent them as 1 3 4 6 => idea
    //This trick makes the run field 1000*1000 => aka smart brute forceable
    let mut all_x_coords: Vec<i64> = positions.clone().iter().map(|pos| pos.x).collect();
    all_x_coords.sort();
    let mut all_x_coords_mapped: Vec<i64> = Vec::new();
    let mut x_to_mapped: HashMap<i64, i64> = HashMap::new();
    let mut mapped_to_x: HashMap<i64, i64> = HashMap::new();
    let mut nextcounter: i64 = 0;
    let mut iter = all_x_coords.iter().peekable();
    while let Some(xpos) = iter.next() {
        all_x_coords_mapped.push(nextcounter.clone());
        x_to_mapped.insert(xpos.clone(), nextcounter.clone());
        mapped_to_x.insert(nextcounter.clone(), xpos.clone());
        nextcounter += 2;
        if iter.peek().is_some() && (*iter.peek().unwrap() - xpos == 1) {
            nextcounter -= 1;
        }
    }

    let mut all_y_coords: Vec<i64> = positions.clone().iter().map(|pos| pos.y).collect();
    all_y_coords.sort();
    let mut all_y_coords_mapped: Vec<i64> = Vec::new();
    let mut y_to_mapped: HashMap<i64, i64> = HashMap::new();
    let mut mapped_to_y: HashMap<i64, i64> = HashMap::new();
    let mut nextcounter: i64 = 0;
    let mut iter = all_y_coords.iter().peekable();
    while let Some(ypos) = iter.next() {
        all_y_coords_mapped.push(nextcounter.clone());
        y_to_mapped.insert(ypos.clone(), nextcounter.clone());
        mapped_to_y.insert(nextcounter.clone(), ypos.clone());
        nextcounter += 1;
        // if iter.peek().is_some() && (*iter.peek().unwrap() - ypos == 1) {
        //     nextcounter -= 1;
        // }
    }

    let mut fielddotzip: Vec<Vec<bool>> =
        vec![
            vec![false; (all_y_coords_mapped.last().unwrap().clone() + 1) as usize];
            (all_x_coords_mapped.last().unwrap().clone() + 1) as usize
        ];

    // println!(
    //     "max x,y zipped {},{}",
    //     all_x_coords_mapped.last().unwrap(),
    //     all_y_coords_mapped.last().unwrap()
    // );

    let positions_mapped: Vec<Position> = positions
        .iter()
        .map(|pos| Position {
            x: x_to_mapped[&pos.x],
            y: y_to_mapped[&pos.y],
        })
        .collect();
    fill_grid(
        &mut fielddotzip,
        positions_mapped.last().unwrap(),
        &positions_mapped[0],
    );
    for positions in positions_mapped.windows(2) {
        fill_grid(&mut fielddotzip, &positions[0], &positions[1]);
    }

    // should be fine
    flood_fill_grid(
        &mut fielddotzip,
        &Position {
            x: (all_y_coords_mapped.last().unwrap().clone() - 3),
            y: (all_y_coords_mapped.last().unwrap().clone() - 3),
        },
    );

    //debug file
    // let mut file = File::create("debug.txt")?;
    // for i in fielddotzip {
    //     for j in i {
    //         file.write(match j {
    //             true => b"x",
    //             false => b"o",
    //         })?;
    //     }
    //     file.write(b"\n")?;
    // }
    let mut result = 0;
    'rectanglecheck: for (area, posidx1, posidx2) in areas {
        let pos1: Position = positions_mapped[posidx1];
        let pos2: Position = positions_mapped[posidx2];

        // println!("handling {} and {}", positions[posidx1], positions[posidx2]);
        //brute force GO
        // correct answer in only 6.3 sec non optimized, 0.19 optimized
        for i in pos1.x.min(pos2.x)..=pos1.x.max(pos2.x) {
            for j in pos1.y.min(pos2.y)..=pos1.y.max(pos2.y) {
                if !fielddotzip[i as usize][j as usize] {
                    continue 'rectanglecheck;
                }
            }
        }
        result = area;
        break;
    }

    println!("the result for day09:2 is {0:?}", result);
    Ok(())
}
