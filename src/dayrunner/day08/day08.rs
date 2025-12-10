use std::{
    cmp,
    fmt::{self},
    fs,
    str::FromStr,
    vec,
};

pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}
#[derive(Clone, Debug, Hash, PartialEq)]
struct Position {
    //i32 to make the pow below easier
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn distance_to(&self, other: &Position) -> u64 {
        //I pray this stays within the u64 limit
        // Checked it, no problem -> welp underflowed still because i was usin u32 as positions
        u64::pow((self.x - other.x).abs() as u64, 2)
            + u64::pow((self.y - other.y).abs() as u64, 2)
            + u64::pow((self.z - other.z).abs() as u64, 2)
    }
}

#[derive(Clone, Debug)]
struct ParseError;
impl FromStr for Position {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let vals: Vec<&str> = s.trim().split(",").collect();
        assert!(vals.len() == 3);
        let mut iter = vals.iter();
        let val1: i32 = iter.next().unwrap().parse().unwrap();
        let val2: i32 = iter.next().unwrap().parse().unwrap();
        let val3: i32 = iter.next().unwrap().parse().unwrap();
        Ok(Position {
            x: val1,
            y: val2,
            z: val3,
        })
    }
}
impl fmt::Display for Position {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let positions = binding
        .lines()
        .map(|line| Position::from_str(line).unwrap())
        .collect::<Vec<Position>>();
    let mut distance_vec: Vec<(u64, &Position, &Position)> = Vec::new();
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            distance_vec.push((
                positions[i].distance_to(&positions[j]),
                &positions[i],
                &positions[j],
            ));
        }
    }
    distance_vec.sort_by(|a, b| a.0.cmp(&b.0));
    let mut graphs: Vec<Vec<&Position>> = Vec::new();
    for i in 0..1000 {
        let handling = distance_vec.get(i).unwrap();
        let mut pos1graphidx: Option<usize> = None;
        let mut pos2graphidx: Option<usize> = None;
        for graphidx in 0..graphs.len() {
            if graphs[graphidx].contains(&handling.1) {
                if !pos1graphidx.is_none() {
                    panic!("position in two graphs, shouldnt happen")
                }
                pos1graphidx = Some(graphidx);
            }
            if graphs[graphidx].contains(&handling.2) {
                if !pos2graphidx.is_none() {
                    panic!("position in two graphs, shouldnt happen")
                }
                pos2graphidx = Some(graphidx);
            }
        }
        if pos1graphidx.is_none() && pos2graphidx.is_none() {
            //create new graph
            graphs.push(vec![handling.1, handling.2]);
        } else if pos1graphidx.is_some() && pos2graphidx.is_none() {
            //add pos2 to graph 1
            graphs[pos1graphidx.unwrap()].push(handling.2);
        } else if pos1graphidx.is_none() && pos2graphidx.is_some() {
            //add pos1 to graph
            graphs[pos2graphidx.unwrap()].push(handling.1);
        } else {
            if pos1graphidx.unwrap() == pos2graphidx.unwrap() {
                continue;
            }
            // if efficiency problems emerge I can try just emptying one of the vecs but keep it there so the vec move doesnt happen
            let mut removed = graphs.remove(cmp::max(pos1graphidx.unwrap(), pos2graphidx.unwrap()));
            //need to merge both graphs
            graphs[cmp::min(pos1graphidx.unwrap(), pos2graphidx.unwrap())].append(&mut removed);
        }
    }
    graphs.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut result: usize = 1;
    // for i in graphs.clone() {
    //     for pos in i {
    //         print!(" {} ", pos);
    //     }
    //     println!();
    // }
    for i in 0..3 {
        result *= graphs[i].len();
    }
    println!("the result for day08:1 is {0:?}", result);
    Ok(())
}
pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let positions = binding
        .lines()
        .map(|line| Position::from_str(line).unwrap())
        .collect::<Vec<Position>>();
    let mut distance_vec: Vec<(u64, &Position, &Position)> = Vec::new();
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            distance_vec.push((
                positions[i].distance_to(&positions[j]),
                &positions[i],
                &positions[j],
            ));
        }
    }
    distance_vec.sort_by(|a, b| a.0.cmp(&b.0));
    let mut graphs: Vec<Vec<&Position>> = Vec::new();
    let mut used_counter = 0;
    let mut i: usize = 0;
    let result_loop: (&Position, &Position) = loop {
        let handling = distance_vec.get(i).unwrap();
        // println!("handling {} {}", handling.1, handling.2);
        i += 1;
        let mut pos1graphidx: Option<usize> = None;
        let mut pos2graphidx: Option<usize> = None;
        for graphidx in 0..graphs.len() {
            if graphs[graphidx].contains(&handling.1) {
                if !pos1graphidx.is_none() {
                    panic!("position in two graphs, shouldnt happen")
                }
                pos1graphidx = Some(graphidx);
            }
            if graphs[graphidx].contains(&handling.2) {
                if !pos2graphidx.is_none() {
                    panic!("position in two graphs, shouldnt happen")
                }
                pos2graphidx = Some(graphidx);
            }
        }
        if pos1graphidx.is_none() && pos2graphidx.is_none() {
            // println!("create");
            //create new graph
            graphs.push(vec![handling.1, handling.2]);
            used_counter += 2;
        } else if pos1graphidx.is_some() && pos2graphidx.is_none() {
            // println!("add");
            //add pos2 to graph 1
            graphs[pos1graphidx.unwrap()].push(handling.2);
            used_counter += 1;
        } else if pos1graphidx.is_none() && pos2graphidx.is_some() {
            // println!("add");
            //add pos1 to graph
            graphs[pos2graphidx.unwrap()].push(handling.1);
            used_counter += 1;
        } else {
            if pos1graphidx.unwrap() == pos2graphidx.unwrap() {
                // println!("skip");
                continue;
            }
            // println!("merge");
            // if efficiency problems emerge I can try just emptying one of the vecs but keep it there so the vec move doesnt happen
            let mut removed = graphs.remove(cmp::max(pos1graphidx.unwrap(), pos2graphidx.unwrap()));
            //need to merge both graphs
            graphs[cmp::min(pos1graphidx.unwrap(), pos2graphidx.unwrap())].append(&mut removed);
        }
        if graphs.len() == 1 && used_counter == positions.len() {
            //return value of loop
            break (handling.1, handling.2);
        }
    };
    let result: i32 = result_loop.0.x * result_loop.1.x;
    // println!("{} {}", result_loop.0, result_loop.1);
    println!("the result for day08:2 is {0:?}", result);
    Ok(())
}
