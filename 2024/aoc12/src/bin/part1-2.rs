use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Plot {
    name: char,
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Region {
    start: Plot,
}

#[derive(Clone, Copy)]
struct Stats {
    area: u32,
    fences: u32,
    sides: u32,
}

/// Initialize the new plot and associate it with a pre-existing region
/// or a new one, according to its position in the garden.
fn init_plot_and_merge(
    new_plot: Plot,
    old_plot: Plot,
    group: &mut HashMap<Plot, Region>,
    region_stats: &mut HashMap<Region, Stats>,
) {
    let new_plot_reg = group.get_mut(&new_plot).cloned();
    let old_plot_reg = *group.get(&old_plot).unwrap();
    if new_plot.name == old_plot.name {
        match new_plot_reg {
            Some(region) => {
                if region.start != old_plot_reg.start {
                    // Two partitions need to be merged: change the region
                    // of the "new region" plots to be the old one
                    if let Some(new_stats) = region_stats.remove(&region) {
                        let old_stats = region_stats.get_mut(&old_plot_reg).unwrap();
                        old_stats.area += new_stats.area;
                        old_stats.fences += new_stats.fences;
                        old_stats.sides += new_stats.sides;
                    }
                    // change the region of each new plot to follow the old one
                    for r in group.values_mut() {
                        if r.start == region.start {
                            r.start = old_plot_reg.start;
                        }
                    }
                    region_stats.remove_entry(&region);
                }
            }
            None => {
                // Add the node to the old region
                group.entry(new_plot).or_insert(old_plot_reg);
                region_stats.entry(old_plot_reg).and_modify(|s| s.area += 1);
            }
        }
    } else {
        // Two different plots, eventually generate its region and increment
        // both fences
        if let Some(new_region) = new_plot_reg {
            region_stats.entry(new_region).and_modify(|s| {
                s.fences += 1;
            });
        } else {
            let new_region = Region { start: new_plot };
            group.entry(new_plot).or_insert(new_region);
            region_stats.entry(new_region).or_insert(Stats {
                area: 1,
                fences: 1,
                sides: 0,
            });
        }
        region_stats
            .entry(old_plot_reg)
            .and_modify(|s| s.fences += 1);
    }
}

/// Finds valid sides near the given reference plot and updates the region
/// statistics.
fn find_new_sides(
    reference: &Plot,
    grid: &[Vec<char>],
    region_stats: &mut HashMap<Region, Stats>,
    group: &HashMap<Plot, Region>,
) {
    // The idea here is to look at the 4x4 square having the reference plot
    // in the bottom-right corner.
    //
    // Having in mind that we proceed from left to right and from top to
    // bottom, the conditions work in this manner:
    // - given our reference plot, we consider the upper plot and the
    //   left plot as opposite plots between each other, and the top-left
    //   plot as the middle one;
    // - a side *can be* a new side only if the front plot is a
    //   different kind of plot
    // +-+-+
    // |X|D|
    // +-+-+ <- the *upper* side of the reference cannot introduce a new
    // |X|D|    side to the region due to the D on top.
    // +-+-+
    //
    // - a side is effectively a new side if the previous condition holds
    //   _and_ one between the following holds:
    //   - the opposite plot is a different kind of plot:
    //
    // +-+-+
    // |X|X|
    // +-+-+ <- the *left* side of the reference cannot introduce a new
    // |D|D|    side to the region due to the D on the left, in this case.
    // +-+-+
    //
    //  OR
    //  - the middle plot is the same kind of plot:
    //
    // +-+-+
    // |D|X|
    // +-+-+ <- this time, the same side is a valid new side of the region.
    // |D|D|
    // +-+-+
    //
    // Comparisons here are done to find which between the following are
    // valid sides:
    // - left side of the reference;
    // - top side of the reference;
    // - bottom side of the top plot;
    // - right side of the left plot.
    //
    // The conditions used for the second half of the list above can be
    // obtained by "mirroring" the plots over the considered side (e.g.
    // for the bottom side of the top plot, the reference is swapped with
    // the top plot and the left plot is swapped with the middle one).
    //
    let left_plot = if reference.y > 0 {
        Some(Plot {
            name: grid[reference.x][reference.y - 1],
            x: reference.x,
            y: reference.y - 1,
        })
    } else {
        None
    };
    let top_plot = if reference.x > 0 {
        Some(Plot {
            name: grid[reference.x - 1][reference.y],
            x: reference.x - 1,
            y: reference.y,
        })
    } else {
        None
    };
    let middle_plot = if left_plot.is_some() && top_plot.is_some() {
        Some(Plot {
            name: grid[reference.x - 1][reference.y - 1],
            x: reference.x - 1,
            y: reference.y - 1,
        })
    } else {
        None
    };
    if let Some(left) = left_plot {
        if let Some(top) = top_plot {
            let middle = middle_plot.unwrap();
            // Upper side of new
            if top.name != reference.name
                && (left.name != reference.name || middle.name == reference.name)
            {
                region_stats
                    .entry(*group.get(reference).unwrap())
                    .and_modify(|s| s.sides += 1);
            }
            // Left side of new
            if left.name != reference.name
                && (top.name != reference.name || middle.name == reference.name)
            {
                region_stats
                    .entry(*group.get(reference).unwrap())
                    .and_modify(|s| s.sides += 1);
            }
            // Lower side of upper
            if reference.name != top.name && (middle.name != top.name || left.name == top.name) {
                region_stats
                    .entry(*group.get(&top).unwrap())
                    .and_modify(|s| s.sides += 1);
            }
            // Right side of left
            if reference.name != left.name && (middle.name != left.name || top.name == left.name) {
                region_stats
                    .entry(*group.get(&left).unwrap())
                    .and_modify(|s| s.sides += 1);
            }
            return;
        }
    }
    // From now on, either top or left is missing.
    if let Some(up) = top_plot {
        // Y == 0
        if up.name != reference.name {
            // Upper and left side of new
            region_stats
                .entry(*group.get(reference).unwrap())
                .and_modify(|s| s.sides += 2);
            // Lower side of top
            region_stats
                .entry(*group.get(&up).unwrap())
                .and_modify(|s| s.sides += 1);
        }
    }
    if let Some(left) = left_plot {
        // X == 0
        if left.name != reference.name {
            // Upper and left side of new
            region_stats
                .entry(*group.get(reference).unwrap())
                .and_modify(|s| s.sides += 2);
            // Right side of left
            region_stats
                .entry(*group.get(&left).unwrap())
                .and_modify(|s| s.sides += 1);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../inputs/input12.txt");
    let input = fs::read_to_string(path)?;
    // Code
    let mut bound = 0;
    let mut group: HashMap<Plot, Region> = HashMap::default();
    let mut region_stats: HashMap<Region, Stats> = HashMap::default();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for (x, l) in grid.iter().enumerate() {
        // Assumption: the grid is square
        let line: Vec<char> = l[..l.len()].into();
        if bound == 0 {
            bound = line.len() - 1;
        }

        for (y, ch) in line.iter().enumerate() {
            let new_plot = Plot { name: *ch, x, y };
            if x == 0 && y == 0 {
                // Checks on the first element of the grid
                let new_region = Region { start: new_plot };
                group.entry(new_plot).or_insert(new_region);
                region_stats.entry(new_region).or_insert(Stats {
                    area: 1,
                    fences: 2,
                    sides: 2,
                });
                continue;
            }
            if y != 0 {
                let left_plot = Plot {
                    name: line[y - 1],
                    x,
                    y: y - 1,
                };
                init_plot_and_merge(new_plot, left_plot, &mut group, &mut region_stats);
            }
            if x != 0 {
                let top_plot = Plot {
                    name: grid[x - 1][y],
                    x: x - 1,
                    y,
                };
                init_plot_and_merge(new_plot, top_plot, &mut group, &mut region_stats);
            }
            find_new_sides(&new_plot, &grid, &mut region_stats, &group);
            if y == 0 || y == bound {
                // Add an extra fence
                let new_region = group.get(&new_plot).unwrap();
                region_stats
                    .entry(*new_region)
                    .and_modify(|s| s.fences += 1);
                // Add an extra side, eventually
                if y == bound && (x == 0 || *ch != grid[x - 1][y]) {
                    region_stats.entry(*new_region).and_modify(|s| s.sides += 1);
                }
            }
            if x == 0 || x == bound {
                // Add an extra fence
                let new_region = group.get(&new_plot).unwrap();
                region_stats
                    .entry(*new_region)
                    .and_modify(|s| s.fences += 1);
                // Add an extra side, eventually
                if x == bound && (y == 0 || *ch != line[y - 1]) {
                    region_stats.entry(*new_region).and_modify(|s| s.sides += 1);
                }
            }
            assert!(group.contains_key(&new_plot));
            assert!(region_stats.contains_key(group.get(&new_plot).unwrap()));
        }
    }
    let cost = &region_stats
        .values()
        .fold(0, |acc, el| acc + (el.area * el.fences));
    println!("Part 1: {cost}");
    // Part 2
    let cost = &region_stats
        .values()
        .fold(0, |acc, el| acc + (el.area * el.sides));
    println!("Part 2: {cost}");
    // Return Ok
    Ok(())
}
