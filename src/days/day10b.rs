use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver,
    variable,
};

use crate::common::{BitVec, parse};

type Button = BitVec<u16>;
type Count = u16;
type State = Vec<Count>;

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
    let mut model = problem.minimise(objective).using(default_solver);
    for (i_machine, &machine_count) in target.iter().enumerate() {
        model = model.with(constraint!(
            p.iter()
                .enumerate()
                .filter_map(|(i_button, v)| {
                    if buttons[i_button].get_bit(i_machine as u16) > 0 {
                        Some(v)
                    } else {
                        None
                    }
                })
                .sum::<Expression>()
                == machine_count
        ));
    }
    let solution = model.solve().unwrap();
    let presses = solution.eval(p.iter().sum::<Expression>());
    #[cfg(debug_assertions)]
    println!("presses = {}", presses);
    presses.round() as usize
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
                j[1..j.len() - 1].split(|&c| c == b',').map(parse).collect()
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

            compute_optimum(target, buttons)
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
