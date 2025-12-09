const NUM_COLUMNS: usize = 141;

pub fn get_result(input: &[u8]) -> usize {
    let mut lines = input
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .step_by(2);
    let first_line = lines.next().unwrap();
    let num_columns = first_line.len();
    let first_pos = num_columns / 2;
    #[cfg(debug_assertions)]
    {
        assert_eq!(
            first_pos,
            first_line.iter().position(|&b| b == b'S').unwrap()
        );
        println!("start pos: {}", first_pos);
    }
    let mut state = [false; NUM_COLUMNS];
    let mut new_state = state.clone();
    state[first_pos] = true;
    let mut split_count = 0;
    for (i, line) in lines.enumerate() {
        #[cfg(debug_assertions)]
        println!(
            "{}",
            state
                .iter()
                .map(|&b| if b { "|" } else { " " })
                .collect::<String>()
        );
        (first_pos - i..=first_pos + i).for_each(|pos| {
            if !state[pos] {
                return;
            }
            match line[pos] {
                b'^' => {
                    split_count += 1;
                    new_state[pos - 1] = true;
                    new_state[pos + 1] = true;
                }
                _ => {
                    new_state[pos] = true;
                }
            };
            state[pos] = false;
        });
        (state, new_state) = (new_state, state);
    }
    #[cfg(debug_assertions)]
    println!(
        "{}",
        state
            .iter()
            .map(|&b| if b { "|" } else { " " })
            .collect::<String>()
    );
    split_count
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day07.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day07.example.txt"));
        assert_eq!(result, 21);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day07.txt"));
        assert_eq!(result, 1615);
    }
}
