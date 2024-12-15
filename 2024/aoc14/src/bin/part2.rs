use regex::Regex;
use std::error::Error;
use std::fs;
//use std::io::stdin;
use std::path::PathBuf;
use utils::Loc;

#[derive(Clone, Copy)]
struct Robot {
    pos: Loc<u32>,
    vel: Loc<i32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input14.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let bounds: Loc<i32> = Loc {
        // Given from puzzle description
        x: 100, //10,
        y: 102, //6
    };
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let px: u32 = caps[1].parse().unwrap();
        let py: u32 = caps[2].parse().unwrap();
        let vx: i32 = caps[3].parse().unwrap();
        let vy: i32 = caps[4].parse().unwrap();
        robots.push(Robot {
            pos: Loc { x: px, y: py },
            vel: Loc { x: vx, y: vy },
        })
    }
    let mut map: Vec<Vec<char>> = Vec::with_capacity(bounds.y as usize + 1);
    for _ in 0..=bounds.y {
        map.push(Vec::with_capacity(bounds.x as usize + 1));
    }
    // let mut cont = true;
    let mut i: u128 = 0;
    loop {
        i += 1;
        let mut found = false;
        for robot in &mut robots {
            // Update position
            robot.pos.x = (robot.pos.x as i32 + robot.vel.x).rem_euclid(bounds.x + 1) as u32;
            robot.pos.y = (robot.pos.y as i32 + robot.vel.y).rem_euclid(bounds.y + 1) as u32;
        }
        // Print
        for y in map.iter_mut().take(bounds.y as usize + 1) {
            for _ in 0..=bounds.x as usize {
                y.push('.');
            }
        }
        for robot in robots.clone() {
            map[robot.pos.y as usize][robot.pos.x as usize] = '#';
        }
        //if cont {
        for (y, line) in map.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                // ez way:
                // if line.iter().collect::<String>().contains("########")
                //
                // what I felt being the most natural way: finding the
                // top of a christmas tree:
                //
                // ....#....
                // ...#.#...
                // ..#...#..
                // .#.....#.
                // #.......#
                //
                // After I found the image by inspection (took some iteration),
                // I just kept adding rows here until the result came out
                // as the first one.
                if map[y][x] == '#'
                    && x >= 4
                    && x <= bounds.x as usize - 4
                    && y <= bounds.y as usize - 4
                    && map[y + 1][x - 1] == '#'
                    && map[y + 1][x + 1] == '#'
                    && map[y + 2][x - 2] == '#'
                    && map[y + 2][x + 2] == '#'
                    && map[y + 3][x - 3] == '#'
                    && map[y + 3][x + 3] == '#'
                    && map[y + 4][x - 4] == '#'
                    && map[y + 4][x + 4] == '#'
                {
                    found = true;
                    break;
                }
            }
        }
        if found {
            // for line in &mut map {
            //     line.clear();
            // }
            //continue;
            break;
        }
        //}
        // This is the code I used to print each iteration. Left here to
        // not forget the sorrow.
        //
        // println!("{i}");
        // let mut buf = String::new();
        // for line in &map {
        //     println!("{}", line.iter().collect::<String>());
        // }
        // println!();
        // let _ = stdin().read_line(&mut buf);
        // if buf.chars().collect::<Vec<char>>()[0] == 'c' {
        //     cont = true;
        // } else if buf.chars().collect::<Vec<char>>()[0] == 'n' {
        //     cont = false;
        // } else if buf.chars().collect::<Vec<char>>()[0] == 'q' {
        //     break;
        // }
        // buf.clear();
        for line in &mut map {
            line.clear();
        }
    }
    println!("{i}");
    // Return Ok
    Ok(())
}
