pub fn get_result(input: &[u8]) -> usize {
    let mut lines = input
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .step_by(2);
    let first_line = lines.next().unwrap();
    let first_pos = first_line.iter().position(|&b| b == b'S').unwrap();
    let mut state = Vec::<usize>::with_capacity(first_line.len());
    let mut new_state = state.clone();
    state.push(first_pos);
    let mut split_count = 0;
    for line in lines {
        #[cfg(debug_assertions)]
        println!("{:?}", state);
        state.iter().for_each(|&pos| {
            match line[pos] {
                b'^' => {
                    split_count += 1;
                    match new_state.last() {
                        Some(&new_pos) if new_pos == pos - 1 => new_state.push(pos + 1),
                        _ => {
                            new_state.push(pos - 1);
                            new_state.push(pos + 1);
                        }
                    };
                }
                _ => {
                    match new_state.last() {
                        Some(&new_pos) if new_pos == pos => {},
                        _ => {
                            new_state.push(pos);
                        }
                    };
                }
            };
        });
        (state, new_state) = (new_state, state);
        new_state.clear();
    }
    #[cfg(debug_assertions)]
    println!("{:?}", state);
    split_count
}

pub fn main() {
    print!(
        "{} ",
        get_result(include_bytes!("../../inputs/day07.txt"))
    );
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
