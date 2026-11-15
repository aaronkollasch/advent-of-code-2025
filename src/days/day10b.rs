use good_lp::{Expression, ProblemVariables, Solution, SolverModel, scip, constraint, Variable, variable};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use hashbrown::HashSet;

use crate::common::{BitVec, parse};

type Button = BitVec<u16>;
type Count = u16;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
struct State {
    prio: Count,
    state: Vec<Count>,
}

impl State {
    fn unpress_button(&self, button: Button) -> Option<Self> {
        let mut new_state = self.state.clone();
        for i in button.iter_set() {
            if new_state[i as usize] == 0 {
                return None;
            }
            new_state[i as usize] -= 1;
        }
        Some(State {
            prio: new_state.iter().sum(),
            state: new_state,
        })
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.prio.cmp(&other.prio)
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

/*
 * for machine i, we can collect the set of buttons that together must be pressed at least n times
 * prioritize buttons that hit a lot of machines
 *
 * machine 0: buttons (0, 1) and (0, 2)
 * machine 1: buttons (1, 3) and (0, 1)
 * machine 2: buttons (2), (2, 3), and (0, 2)
 * machine 3: buttons (3), (1, 3), and (2, 3)
 *
 * minimize button press sum p_i
 * subject to sum p_i * b_i = j
 *
 * sub-problem:
 * given current state j_i, how can we add some minimum combination of p_i such that j_i + sum p_i * b_i = j_final?
 *
 * working from the final node j_final, let the quality heuristic be sum(j)
 * push states to the heap, adding 1 to each p_i and updating j, excluding any that are out of bounds (any j < 0)
 */

fn compute_optimum(target: State, buttons: Vec<Button>) -> usize {
    let mut problem = ProblemVariables::new();
    // p: presses for each button i
    let p: Vec<Variable> = problem.add_vector(variable().integer().min(0), buttons.len());
    // minimize total button presses
    let objective: Expression = p.iter().sum();
    let mut model = problem.minimise(objective).using(scip);
    for i_machine in 0..target.state.len() {
        target.state[i_machine];
        model = model.with(constraint!(
            p.iter().enumerate().filter_map(|(i_button, v)| {
                if buttons[i_button].get_bit(i_machine as u16) > 0 {
                    Some(v)
                } else { None }
            }).sum::<Expression>() == target.state[i_machine]
        ));
    }
    let solution = model.solve().unwrap();
    let presses = solution.eval(p.iter().sum::<Expression>());
    #[cfg(debug_assertions)]
    println!("presses = {}", presses);
    return presses.round() as usize;

    // let mut optimums: HashSet<State> = HashSet::with_capacity(1024);
    // let mut heap: HashSet<State> = HashSet::with_capacity(100);
    // let mut new_heap: HashSet<State> = heap.clone();
    // heap.insert(target.clone());
    // for press in 0.. {
    //     let mut best_prio = target.prio;
    //     #[cfg(debug_assertions)]
    //     println!("press: {}", press);
    //     for prev_state in heap.drain() {
    //         #[cfg(debug_assertions)]
    //         println!("{:?}", prev_state);
    //         if prev_state.prio == 0 {
    //             println!("{:?} {}", target, press);
    //             return press;
    //         }
    //         buttons.iter().for_each(|&b| {
    //             if let Some(new_state) = prev_state.unpress_button(b) {
    //                 best_prio = best_prio.min(new_state.prio);
    //                 if !optimums.contains(&new_state) {
    //                     optimums.insert(new_state.clone());
    //                     new_heap.insert(new_state);
    //                 }
    //             }
    //         });
    //     }
    //     (heap, new_heap) = (new_heap, heap);
    //     heap.retain(|s| s.prio <= best_prio + buttons.len() as u16 * 3);
    //     if heap.is_empty() {
    //         panic!("no solution found! {:?}", target)
    //     }
    // }
    usize::MAX
}

pub fn get_result(input: &[u8]) -> usize {
    input
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let mut iter = l.split(|&c| c == b' ');
            let indicators = iter.next().unwrap();
            let n_buttons = indicators.len() - 2;
            let mut buttons = iter.collect::<Vec<_>>();
            let target: Vec<Count> = {
                let j = buttons.pop().unwrap();
                j[1..j.len() - 1]
                    .split(|&c| c == b',')
                    .map(|num| parse(num))
                    .collect()
            };
            let buttons: Vec<Button> = {
                buttons
                    .iter()
                    .map(|&b| {
                        let mut button: Button = BitVec::new(n_buttons);
                        b[1..b.len() - 1]
                            .split(|&c| c == b',')
                            .for_each(|num| button.set_bit(parse(num)));
                        button
                    })
                    .collect()
            };
            #[cfg(debug_assertions)]
            println!("{:?}, {:?}", target, buttons);

            compute_optimum(
                State {
                    prio: target.iter().sum(),
                    state: target,
                },
                buttons,
            )
        })
        .sum()
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day10.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day10.example.txt"));
        assert_eq!(result, 33);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day10.txt"));
        assert_eq!(result, 20617);
    }
}
