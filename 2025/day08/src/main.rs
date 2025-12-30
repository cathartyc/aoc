use std::{
    cell::RefCell,
    error::Error,
    rc::{Rc, Weak},
};

use utils::get_input;

struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
    circuit: Option<WeakCircuitRef>,
}

impl JunctionBox {
    /// Computes the euclidean distance between this junction box and another one.
    fn distance(&self, to: &Self) -> f64 {
        ((self.x.abs_diff(to.x).pow(2)
            + self.y.abs_diff(to.y).pow(2)
            + self.z.abs_diff(to.z).pow(2)) as f64)
            .sqrt()
    }
}

type JunctionRef = Rc<RefCell<JunctionBox>>;
type WeakJunctionRef = Weak<RefCell<JunctionBox>>;

struct JunctionCouple(JunctionRef, JunctionRef);

/// Collection of junction boxes
type Circuit = Vec<WeakJunctionRef>;
type CircuitRef = Rc<RefCell<Circuit>>;
type WeakCircuitRef = Weak<RefCell<Circuit>>;

/// Retrieve the junction boxes from the input.
fn parse_junction_boxes(input: &str) -> Vec<JunctionRef> {
    input
        .lines()
        .map(|l| {
            let mut parsed_line = l.split(',').map(|coord| coord.parse::<u64>().unwrap());
            Rc::new(RefCell::new(JunctionBox {
                x: parsed_line.next().unwrap(),
                y: parsed_line.next().unwrap(),
                z: parsed_line.next().unwrap(),
                circuit: None,
            }))
        })
        .collect()
}

/// Build a vector with pairs of junction boxes, sorted by distance
fn build_junction_couples(junction_boxes: &Vec<JunctionRef>) -> Vec<JunctionCouple> {
    let mut junction_couples = junction_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            junction_boxes
                .iter()
                .skip(i + 1)
                .map(|b| JunctionCouple(Rc::clone(a), Rc::clone(b)))
        })
        .collect::<Vec<JunctionCouple>>();
    // Given that we are already here, let's sort them
    junction_couples.sort_unstable_by(|couple_1, couple_2| {
        (*couple_1.0.borrow())
            .distance(&*couple_1.1.borrow())
            .total_cmp(&(*couple_2.0.borrow()).distance(&couple_2.1.borrow()))
    });
    junction_couples
}

fn merge_couple(couple: &JunctionCouple, circuits: &mut Vec<CircuitRef>) {
    let j1 = &mut *couple.0.borrow_mut();
    let j2 = &mut *couple.1.borrow_mut();
    // check the belonging of each box to a circuit
    match (j1.circuit.clone(), j2.circuit.clone()) {
        (Some(c1), Some(c2)) => {
            // merge if not already within the same circuit
            if c1.as_ptr() == c2.as_ptr() {
                return;
            }

            // Extract circuits from weak references
            // Don't ask me why I needed to do this in two different statements...
            let circ1 = c1.upgrade().unwrap();
            let mut circ1 = circ1.borrow_mut();
            // circuit 2 requires extra step
            let circ2 = c2.upgrade().unwrap();
            let idx = circuits
                .iter()
                .position(|c| c.as_ptr() == circ2.as_ptr())
                .unwrap();
            let circ2 = circuits.remove(idx);

            // Update boxes in circuit 2 to point to circuit 1
            // and add them into circuit 1 as well
            for jb in circ2.borrow_mut().iter_mut() {
                if jb.as_ptr() != couple.1.as_ref() {
                    (*jb.upgrade().unwrap().borrow_mut()).circuit = Some(c1.clone());
                    circ1.push(jb.clone());
                }
            }

            // Add box 2 into the circuit 1
            circ1.push(Rc::downgrade(&couple.1));
            j2.circuit = Some(c1.clone());
        }
        (Some(c1), None) => {
            // add box 2 into circuit 1
            c1.upgrade()
                .unwrap()
                .borrow_mut()
                .push(Rc::downgrade(&couple.1));
            j2.circuit = Some(c1.clone());
        }
        (None, Some(c2)) => {
            // like before, but with swapped roles
            c2.upgrade()
                .unwrap()
                .borrow_mut()
                .push(Rc::downgrade(&couple.0));
            j1.circuit = Some(c2.clone());
        }
        (None, None) => {
            // make a new circuit
            let circuit = Rc::new(RefCell::new(vec![
                Rc::downgrade(&couple.0),
                Rc::downgrade(&couple.1),
            ]));
            circuits.push(circuit.clone());
            j1.circuit = Some(Rc::downgrade(&circuit));
            j2.circuit = Some(Rc::downgrade(&circuit));
        }
    }
}

fn part_1(input: &str, is_test: bool) -> u64 {
    // Parse junction boxes
    let junction_boxes = parse_junction_boxes(input);

    // Make couples
    let junction_couples = build_junction_couples(&junction_boxes);

    let amount = if is_test { 10 } else { 1000 };
    let mut circuits: Vec<CircuitRef> = vec![];

    // Get the first [10,1000] couples and group them as they come
    for couple in junction_couples[0..amount].iter() {
        merge_couple(couple, &mut circuits);
    }

    // Get the 3 largest circuits
    circuits.sort_unstable_by_key(|c| c.borrow().len());
    circuits
        .iter()
        .rev() // default is ascending order so we need this
        .take(3)
        .map(|c| c.borrow().len())
        .product::<usize>() as u64
}

fn part_2(input: &str) -> u64 {
    // Parse junction boxes
    let junction_boxes = parse_junction_boxes(input);

    // Make couples
    let junction_couples = build_junction_couples(&junction_boxes);

    // Create the minimum spanning tree
    let mut circuits: Vec<CircuitRef> = vec![];
    for couple in junction_couples.iter() {
        merge_couple(couple, &mut circuits);
        // Detect last merge and return the result
        if circuits.len() == 1 && circuits[0].borrow().len() == junction_boxes.len() {
            return (couple.0.borrow().x * couple.1.borrow().x) as u64;
        }
    }
    0
}

const PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input(PATH, false);
    let result_pt1 = part_1(&input, false);
    println!("[Part 1] The result is {result_pt1}.");
    let result_pt2 = part_2(&input);
    println!("[Part 2] The result is {result_pt2}.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part_1(&get_input(PATH, true), true), 40);
    }
    #[test]
    fn test_2() {
        assert_eq!(part_2(&get_input(PATH, true)), 25272);
    }
}
