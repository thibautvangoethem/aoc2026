use std::collections::HashMap;
use std::fs;

pub fn solve(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    solve1(path)?;
    solve2(path)?;
    Ok(())
}

#[derive(Clone, Debug)]
struct Visitor {
    pos: (usize, usize),
    // splitters: Vec<(usize, usize)>,
}

pub fn solve1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.lines();

    let field: Vec<Vec<char>> = contents.map(|line| line.chars().collect()).collect();
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let startpos: (usize, usize) = (0, field[0].iter().position(|c| *c == 'S').unwrap());
    let mut tovisit: Vec<Visitor> = vec![Visitor { pos: startpos }];

    let mut splits = 0;
    while !tovisit.is_empty() {
        let handling = tovisit.pop().unwrap();

        match field
            .get(handling.pos.0)
            .and_then(|row| row.get(handling.pos.1))
        {
            Some('S' | '.') => {
                if !visited.contains(&handling.pos) {
                    visited.push(handling.pos.clone());
                    tovisit.push(Visitor {
                        pos: (handling.pos.0 + 1, handling.pos.1),
                    });
                }
            }
            Some('^') => {
                splits += 1;
                tovisit.push(Visitor {
                    pos: (handling.pos.0 + 1, handling.pos.1 - 1),
                });

                tovisit.push(Visitor {
                    pos: (handling.pos.0 + 1, handling.pos.1 + 1),
                });
            }
            _ => {
                //end of line or out of bounds, nothing to be done anymore
            }
        }
    }
    println!("the result for day07:1 is {0:?}", splits);
    Ok(())
}
#[derive(Clone, Debug)]
struct VisitorAdvanced {
    pos: (usize, usize),
    splitters: Vec<(usize, usize)>,
}

pub fn solve2(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    //Honestly looking back this code feels very inneficient, does a bit of caching/dp but really not far enough.
    //Feels also like a mathematical solution should be there
    //That being said my pc runs this in 0.17s, so rust even handles my badly optimised code quite well
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents = binding.lines();
    let mut splitter: HashMap<(usize, usize), u64> = HashMap::new();

    let field: Vec<Vec<char>> = contents.map(|line| line.chars().collect()).collect();
    let startpos: (usize, usize) = (0, field[0].iter().position(|c| *c == 'S').unwrap());
    splitter.insert(startpos, 1);
    let mut tovisit: Vec<VisitorAdvanced> = vec![VisitorAdvanced {
        pos: startpos,
        splitters: vec![startpos],
    }];
    while !tovisit.is_empty() {
        let mut handling = tovisit.pop().unwrap();

        match field
            .get(handling.pos.0)
            .and_then(|row| row.get(handling.pos.1))
        {
            Some('S' | '.') => {
                handling.pos.0 += 1;
                tovisit.push(handling);
            }
            Some('^') => {
                if splitter.contains_key(&handling.pos) {
                    //case split already evaled before
                    //clone required for borrow checker (mut get ;ater), not a problem as splitters never create loops
                    let valhandled = splitter.get(&handling.pos).unwrap().clone();
                    //clone here should become while loop, clone very inneficient
                    for update in handling.splitters.clone() {
                        *splitter.get_mut(&update).unwrap() += valhandled;
                    }
                } else {
                    // println!("splitting {},{}", handling.pos.0, handling.pos.1);
                    //case split still to be done
                    for update in handling.splitters.clone() {
                        *splitter.get_mut(&update).unwrap() += 1;
                    }

                    splitter.insert(handling.pos, 1);

                    handling.splitters.push(handling.pos);
                    handling.pos.0 += 1;
                    handling.pos.1 += 1;
                    tovisit.push(handling.clone());

                    handling.pos.1 -= 2;
                    tovisit.push(handling);
                }
            }
            _ => {
                //end of line or out of bounds, nothing to be done anymore
            }
        }
    }
    println!(
        "the result for day07:2 is {0:?}",
        splitter.get(&startpos).unwrap()
    );
    Ok(())
}
